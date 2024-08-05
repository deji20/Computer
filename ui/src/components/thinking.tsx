import { ReactNode } from "react";

interface ThinkingProps{
    children?: ReactNode
    className?: string
}

export default function Thinking(props: ThinkingProps){
    return (
        <div className={`absolute h-screen w-screen flex ${props.className}`}>
            <div className="animate-spin rounded-full h-48 w-48 m-auto border-t-2 border-b-2 border-sky-500"></div>
            {props.children}
        </div>
    )
}