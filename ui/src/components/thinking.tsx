import { ReactNode } from "react";
import { motion } from "framer-motion";

interface ThinkingProps{
    children?: ReactNode
    className?: string
}

export default function Thinking(props: ThinkingProps){
    return (
        <motion.div
            initial={{opacity: 0}}
            animate={{opacity: 1}}
            exit={{opacity: 0}}
            className={`${props.className}`}>
                {props.children}
        </motion.div>
    )
}