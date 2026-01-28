import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { motion } from "framer-motion";
import { invoke } from "@tauri-apps/api/core";
import { pictureDir } from "@tauri-apps/api/path";
import { path } from "@tauri-apps/api";

import { Layout, useData } from "../../Contexts/DataContext";

import './styles.css'

function Countdown() {
  const navigate = useNavigate();
  const [count, setCount] = useState(3);
  const [photoIndex, setPhotoIndex] = useState(1)
  const [isStarting, setIsStarting] = useState(true)
  const { options, setImages } = useData();

  useEffect(() => {
    const startDelay = setTimeout(() => {
      setIsStarting(false)
    }, 3000);

    return () => clearTimeout(startDelay)
  }, [])

  useEffect(() => {
    if (isStarting) return

    let photo_num = options.layout == Layout.A ? 1 : 4;

    if (count === 0 && photoIndex <= photo_num) {
      async function capturePhoto() {
        const pictures = await pictureDir();
        try {
          let img_path = await path.join(pictures, `photo-${photoIndex}.jpg`)
          let img = await invoke<string>("capture", {
            outputPath: img_path,
            layout: options.layout
          });
          setImages(prev => [...prev, img]);
        } catch (err) {
          console.error("Failed to capture image:", err);
        }
      }

      capturePhoto().then(() => {
        if (photoIndex < photo_num) {
          setTimeout(() => {
            setPhotoIndex(prev => prev + 1);
            setCount(3);
          }, 1000);
        } else {
          setTimeout(() => {
            navigate(options.digital ? "/mail" : "/greeting");
          }, 1000);
        }
      });
    }
  }, [count, photoIndex, navigate, options.digital, setImages, isStarting]);

  useEffect(() => {
    if (isStarting || count <= 0) return

    if (count > 0) {
      const timer = setTimeout(() => {
        setCount(prev => prev - 1);
      }, 1000);

      return () => clearTimeout(timer);
    }
  }, [count, isStarting]);

  return (
    <div id="countdown">
      {isStarting ? (
        <motion.span
          key="starting"
          className="count-starting"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          transition={{ duration: 0.5 }}
        >
          Starting countdown...
        </motion.span>
      ) : (
        count > 0 && (
          <motion.span
            key={count}
            className="count"
            initial={{ opacity: 1, scale: 1 }}
            animate={{ opacity: 0, scale: 2 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 1, ease: "easeInOut" }}
          >
            {count}
          </motion.span>
        )
      )}
      {!isStarting && <span className="count-text">
        Choose a pose now, stay still after 1...
      </span>}
    </div>
  );
}

export default Countdown;