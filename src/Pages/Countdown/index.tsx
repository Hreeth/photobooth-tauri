import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { motion } from "framer-motion";

import { useData } from "../../Contexts/DataContext";

import './styles.css'
import { invoke } from "@tauri-apps/api/core";

function Countdown() {
  const navigate = useNavigate();
  const [count, setCount] = useState(5);
  const [photoIndex, setPhotoIndex] = useState(1)
  const { options } = useData();

  useEffect(() => {
    if (photoIndex <= 4) {
      if (count > 0) {
        const timer = setTimeout(() => {
          setCount(prev => prev - 1)
        }, 1000);

        return () => clearTimeout(timer)
      } else {
        invoke("capture", { outputPath: `photo_${photoIndex}.jpg`, index: photoIndex })
          .then(() => {
            console.log(`Photo ${photoIndex} taken successfully`);
          })
          .catch((err) => {
            console.error(`Error taking photo ${photoIndex}:`, err);
          });

        if (photoIndex < 4) {
          setCount(5);
          setPhotoIndex((prev) => prev + 1);
        } else {
          navigate(options.digital ? "/mail" : "/greeting");
        }
      }
    }
  }, [count, navigate, options.digital, photoIndex]);

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