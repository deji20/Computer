import { useEffect, useState } from "react";
import { motion } from "framer-motion";

interface AnimatedSoundProps {
    sound: number;
    className?: string;
}

export default function AnimatedSound(props: AnimatedSoundProps) {
    const [waves, setWaves] = useState<number[]>(Array(50).fill(0));

    useEffect(() => {
        if(waves.length < 50) setWaves([...waves, props.sound]);
        else setWaves([...waves.slice(1), props.sound]);
    }, [props.sound]);
    return (
        <div className={"w-full h-full relative flex " + props.className}>
            <div className="m-auto relative flex">
            {waves.map((wave, index) =>
                <motion.div key={index} className="absolute text-center m-auto border bg-sky-200 p-1  flex rounded-full shadow-xl "
                animate={{
                    top: (Math.sin((waves.length-1)/360 * (index))) * (100 + (200 * wave)) + "px", 
                    left: (Math.cos((waves.length-1)/360 * (index))) * (100 + (200 * wave)) + "px",  
                }}
                // animate={{ y: -(wave * 100).toFixed(3) + "px" }}
                >
                {/* <p className="m-auto text-black">{(wave).toFixed(4)}</p> */}
                </motion.div>
            )}
            </div>
        </div>
    )
}