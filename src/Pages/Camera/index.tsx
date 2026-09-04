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
  const countdownId = useRef(0);

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

  const beginCountdown = useCallback(() => {
    cleanupCountdown();
    cleanupFlash();

    countdownId.current += 1;

    const id = countdownId.current;

    setFlash(false);
    setPreview(null);
    setCount(5);
    setState("countdown");

    const tick = (current: number) => {
      if (id !== countdownId.current) {
        return;
      }

      if (current === 0) {
        const doCapture = async () => {
          if (capturing.current || id !== countdownId.current) {
            return;
          }

          capturing.current = true;

          try {
            await capture();
          } catch (err) {
            console.error("Failed to capture image:", err);

            if (id === countdownId.current) {
              setState("waiting");
            }
          } finally {
            capturing.current = false;
          }
        };

        doCapture();
        return;
      }

      setCount(current);

      countdownTimeout.current = setTimeout(() => {
        tick(current - 1);
      }, 1000);
    };

    tick(5);
  }, [cleanupCountdown, cleanupFlash]);

  const handleStart = () => {
    beginCountdown();
  };

  const handleRetake = async () => {
    try {
      await retake();
      setRetakes((current) => current + 1);
      beginCountdown();
    } catch (err) {
      console.error("Failed to retake photo:", err);
    }
  };

  const handleNext = async () => {
    try {
      const complete = await acceptPhoto();

      if (complete) {
        setState("processing");

        await processSession();

        await stopCamera();

        navigate(
          options.digital ? "/mail" : "/greeting",
        );

        return;
      }

      setRetakes(0);
      beginCountdown();
    } catch (err) {
      console.error("Failed to accept photo:", err);
    }
  };

  useEffect(() => {
    return () => {
      countdownId.current += 1;
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
            <div className="attempts-remaining">
              <b>Attempts Remaining:</b> {2 - retakes}
            </div>

            <div>
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
            </div>

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