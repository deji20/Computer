import { motion, AnimatePresence } from "framer-motion";
import { useEffect, useState } from "react";

interface Chat{
    word: string,
    sentence: string,
    index: number,
}

export default function AnimatedText(props: { text: string }) {
    const [chat, setChat] = useState<Chat>({
        word: "",
        sentence: "",
        index: 0,
    });

    useEffect(() => {
        const w = chat.word + props.text;
        if(w.split(" ").length >= 3 && w[w.length - 1] != " ") 
          setChat({...chat, word: w.split(" ").slice(2).join(" "), index: chat.index + 1 });
        else setChat({...chat, word: w });
      }, [props.text]);

    return (
        <motion.div
            initial={{ background: "rgba(0, 0, 0, 0)",}} 
            animate={{ background: "rgba(0, 0, 0, 0.5)", }}
            className="absolute top-0 left-0 w-full h-full bg-black bg-opacity-50 flex justify-center items-center">
        <div className="m-auto absolute flex">
            <AnimatePresence>
            <motion.p className="relative"
                key={"word_" + chat.index} transition={{
                //smooth out the animation
                type: "spring",
                stiffness: 160,
                damping: 20,
                mass: 1,
                restDelta: 0.5,
                restSpeed: 0.5,
                }}
                animate={{ top: 0, opacity: 1 }} initial={{ top: 20, opacity: 1 }} exit={{ position:"absolute", opacity: 0, top: -20 }}>
                <AnimatePresence>
                {[...chat.word].map((token, index) => {
                return (
                    <motion.span key={token + index + chat.index} transition={{
                    //smooth out the animation
                    type: "spring",
                    stiffness: 160,
                    damping: 20,
                    mass: 1,
                    restDelta: 0.5,
                    restSpeed: 0.5,
                    delay: index * 0.08,
                    }}
                    animate={{ top: 0, opacity: 1 }} initial={{ top: 10, opacity: 0 }} exit={{ top: -10, opacity: 0 }}
                    className="relative p-[1px]">
                    {token}
                    </motion.span>)
                    })}
                </AnimatePresence>
            </motion.p>
            </AnimatePresence>
        </div>
    </motion.div>
)}