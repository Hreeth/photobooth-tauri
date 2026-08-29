import { useCallback, useEffect, useRef, useState } from "react";

import { useNavigate } from "react-router-dom";

import { motion, AnimatePresence } from "framer-motion";

import { listen, type UnlistenFn } from "@tauri-apps/api/event";

import { useData } from "../../Contexts/DataContext";

import {
  acceptPhoto,
  capture,
  processSession,
  retake,
  startCamera,
  stopCamera,
} from "../../Services/commands";

import "./styles.css";

type CaptureState =
  | "waiting"
  | "countdown"
  | "reviewing"
  | "processing";

export default function Camera() {
  const navigate = useNavigate();
  const { options } = useData();

  const [state, setState] = useState<CaptureState>("waiting");
  const [count, setCount] = useState(5);
  const [liveFrame, setLiveFrame] = useState<string | null>(null);
  const [preview, setPreview] = useState<string | null>(null);
  const [retakes, setRetakes] = useState(0);
  const [flash, setFlash] = useState(false);

  const countdownTimeout = useRef<
    ReturnType<typeof setTimeout> | null
  >(null);

  const flashTimeout = useRef<
    ReturnType<typeof setTimeout> | null
  >(null);

  const capturing = useRef(false);

  const cleanupCountdown = useCallback(() => {
    if (countdownTimeout.current !== null) {
      clearTimeout(countdownTimeout.current);
      countdownTimeout.current = null;
    }
  }, []);

  const cleanupFlash = useCallback(() => {
    if (flashTimeout.current !== null) {
      clearTimeout(flashTimeout.current);
      flashTimeout.current = null;
    }
  }, []);

  /*
   * Continuous camera preview.
   *
   * The listener is registered before the camera starts so
   * we cannot miss the first camera frames.
   *
   * Backend:
   *   camera-frame -> base64 JPEG
   */
  useEffect(() => {
    let unlisten: UnlistenFn | undefined;

    const setup = async () => {
      unlisten = await listen<string>("camera-frame", (event) => {
        setLiveFrame(
          `data:image/jpeg;base64,${event.payload}`,
        );
      });

      try {
        await startCamera();
      } catch (err) {
        console.error("Failed to start camera:", err);
      }
    };

    setup();

    return () => {
      unlisten?.();

      stopCamera().catch((err) => {
        console.error("Failed to stop camera:", err);
      });
    };
  }, []);

  /*
   * Captured photo preview.
   *
   * Backend:
   *   take-preview -> base64 JPEG
   *
   * The frontend does not keep the captured file.
   * It only keeps the base64 preview needed for display.
   */
  useEffect(() => {
    let unlisten: UnlistenFn | undefined;

    const setup = async () => {
      unlisten = await listen<string>("take-preview", (event) => {
        setPreview(
          `data:image/jpeg;base64,${event.payload}`,
        );

        setState("reviewing");

        cleanupFlash();

        setFlash(true);

        flashTimeout.current = setTimeout(() => {
          setFlash(false);
          flashTimeout.current = null;
        }, 120);
      });
    };

    setup();

    return () => {
      unlisten?.();
    };
  }, [cleanupFlash]);

  /*
   * Begin a new countdown.
   */
  const beginCountdown = useCallback(() => {
    cleanupCountdown();
    cleanupFlash();

    setFlash(false);
    setPreview(null);
    setCount(5);
    setState("countdown");
  }, [cleanupCountdown, cleanupFlash]);

  /*
   * Start button.
   *
   * The camera is already running.
   * This only begins the capture countdown.
   */
  const handleStart = () => {
    beginCountdown();
  };

  /*
   * Countdown and capture.
   */
  useEffect(() => {
    if (state !== "countdown") {
      return;
    }

    if (count === 0) {
      const doCapture = async () => {
        if (capturing.current) {
          return;
        }

        capturing.current = true;

        try {
          /*
           * capture() waits for the backend capture operation
           * to finish.
           *
           * The backend emits take-preview containing the
           * base64 preview.
           *
           * The take-preview event changes the state to
           * reviewing once the preview is available.
           */
          await capture();
        } catch (err) {
          console.error("Failed to capture image:", err);
          setState("waiting");
        } finally {
          capturing.current = false;
        }
      };

      doCapture();
      return;
    }

    countdownTimeout.current = setTimeout(() => {
      setCount((current) => current - 1);
    }, 1000);

    return cleanupCountdown;
  }, [count, state, cleanupCountdown]);

  /*
   * Retake the current photo.
   *
   * Backend enforces the maximum number of retakes.
   */
  const handleRetake = async () => {
    try {
      await retake();

      setRetakes((current) => current + 1);

      await beginCountdown();
    } catch (err) {
      console.error("Failed to retake photo:", err);
    }
  };

  /*
   * Accept the current photo.
   */
  const handleNext = async () => {
    try {
      const complete = await acceptPhoto();

      if (complete) {
        setState("processing");

        /*
         * processSession waits for the backend processing to finish.
         *
         * The heavy image processing runs on a blocking worker
         * thread, so the window remains responsive while we wait.
         *
         * This resolves only after:
         *   - all photos are opened
         *   - filters are applied
         *   - composition is created
         *   - bleed is applied
         *   - final.jpg is saved
         *   - session.final is updated
         */
        await processSession();

        await stopCamera();

        navigate(
          options.digital ? "/mail" : "/greeting",
        );

        return;
      }

      setRetakes(0);

      await beginCountdown();
    } catch (err) {
      console.error("Failed to accept photo:", err);
    }
  };

  /*
   * Cleanup timers when leaving the page.
   */
  useEffect(() => {
    return () => {
      cleanupCountdown();
      cleanupFlash();
    };
  }, [cleanupCountdown, cleanupFlash]);

  const showLivePreview =
    state === "waiting" || state === "countdown";

  const showCapturedPreview =
    state === "reviewing" && preview !== null;

  return (
    <motion.div
      id="camera"
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
    >
      {state !== "processing" && (
        <div className="camera-preview">
          {showLivePreview && liveFrame && (
            <img
              className="camera-frame"
              src={liveFrame}
              alt="Live camera preview"
            />
          )}

          {showCapturedPreview && preview && (
            <img
              className="camera-frame"
              src={preview}
              alt="Captured preview"
            />
          )}

          {!liveFrame && state !== "reviewing" && (
            <div className="camera-placeholder" />
          )}

          {flash && (
            <div className="camera-flash" />
          )}

          <AnimatePresence>
            {state === "waiting" && (
              <motion.div
                className="camera-start-overlay"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
              >
                <div className="camera-start-content">
                  <p>
                    Take your time.
                    <br />
                    The camera can wait.
                  </p>

                  <button
                    type="button"
                    onClick={handleStart}
                    className="start-button"
                  >
                    Start
                  </button>
                </div>
              </motion.div>
            )}
          </AnimatePresence>
        </div>
      )}

      <div className="capture-controls">
        {state === "countdown" && (
          <>
            <motion.div
              key={count}
              className="count"
              initial={{
                opacity: 0,
                scale: 0.8,
              }}
              animate={{
                opacity: 1,
                scale: 1,
              }}
              transition={{
                duration: 0.2,
              }}
            >
              {count}
            </motion.div>

            <div className="count-text">
              Choose a pose now, stay still after 1...
            </div>
          </>
        )}

        {state === "reviewing" && (
          <motion.div
            className="review-controls"
            initial={{
              opacity: 0,
              y: 10,
            }}
            animate={{
              opacity: 1,
              y: 0,
            }}
          >
            <button
              type="button"
              onClick={handleRetake}
              disabled={retakes >= 2}
              className="retake-button"
            >
              Retake
            </button>

            <button
              type="button"
              onClick={handleNext}
              className="next-button"
            >
              Next
            </button>
          </motion.div>
        )}

        {state === "processing" && (
          <div className="processing-text">
            Preparing your photos...
          </div>
        )}
      </div>
    </motion.div>
  );
}