import { motion, AnimatePresence } from "framer-motion";
import { useEffect, useState } from "react";

interface Chat{
    word: string,
    sentence: string,
    index: number,
}
interface AnimatedTextProps {
    className?: string;
    text: string,
    sustained?: boolean,
}

export default function AnimatedText(props: AnimatedTextProps) {
    const [chat, setChat] = useState<Chat>({
        word: "",
        sentence: "",
        index: 0,
    });

    useEffect(() => {
        if(!props.sustained) {
            setChat({sentence: props.text, word: props.text, index: chat.index + 1 });
            return;
        }
        const w = chat.word + props.text;
        if(w.split(" ").length >= 3 && w[w.length - 1] != " ") 
          setChat({...chat, word: w.split(" ").slice(2).join(" "), index: chat.index + 1 });
        else setChat({...chat, word: w });
      }, [props.text]);

    return (
    <div className={props.className}>
        <AnimatePresence>
            <motion.p className={"w-min h-min text-gray-300 font-thin relative flex m-auto"}
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
                {chat.word.split("").map((token, index) => {
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
)}