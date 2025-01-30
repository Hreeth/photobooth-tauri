import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { motion } from "framer-motion";

import { useData } from "../../Contexts/DataContext";

import './styles.css'
import { invoke } from "@tauri-apps/api/core";
import { pictureDir } from "@tauri-apps/api/path";

function Countdown() {
  const navigate = useNavigate();
  const [count, setCount] = useState(5);
  const [photoIndex, setPhotoIndex] = useState(1)
  const { options, setImages } = useData();

  useEffect(() => {
    if (count === 0 && photoIndex <= 4) {
      async function capturePhoto() {
        const pictures = await pictureDir();
        try {
          let img = await invoke<string>("capture", { outputPath: `${pictures}/photo-${photoIndex}.jpg` });
          setImages(prev => [...prev, img]);
        } catch (err) {
          console.error("Failed to capture image:", err);
        }
      }

      capturePhoto().then(() => {
        if (photoIndex < 4) {
          setTimeout(() => {
            setPhotoIndex(prev => prev + 1);
            setCount(5);
          }, 1000);
        } else {
          setTimeout(() => {
            navigate(options.digital ? "/mail" : "/greeting");
          }, 1000);
        }
      });
    }
  }, [count, photoIndex, navigate, options.digital, setImages]);

  useEffect(() => {
    if (count > 0) {
      const timer = setTimeout(() => {
        setCount(prev => prev - 1);
      }, 1000);

      return () => clearTimeout(timer);
    }
  }, [count]);
 
  return (
    <div id="countdown">
        {count > 0 && (
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
        )}
    </div>
  );
}

export default Countdown;