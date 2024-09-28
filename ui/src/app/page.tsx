'use client';

import { useEffect, useState } from "react";
import MoviePage from "./display/movies";
import UseBrain from "./brain/useBrain";
import AnimatedText from "../components/animatedText";
import Thinking from "../components/thinking";
import { AnimatePresence } from "framer-motion";
import AnimatedSound from "@/components/animatedSound";
import { invoke } from '@tauri-apps/api/tauri';


export default function Home() {
  useEffect(() => { invoke("start") }, [])
  const [text, setText] = useState<string>("hello world");
  const [display, setDisplay] = useState<{page: "Movies" | "Home", props: any,}>({ page: "Home", props: {

  } });
  const [error, setError] = useState<string>("");
  
  const [wave, setWave] = useState<number>(0)

  const brain = UseBrain({
      onThought: (thought) => setText(thought),
      onMovieList: (movies) => { console.log(movies); setDisplay({props: movies, page: "Movies"}) },
      onError: (error) => setError(error),
      onSpeech: (speech) => setText(speech),
      onSelect: (choice) => {},
      onListen: (audio) => setWave(audio) 
  })

  return (
    <main className="flex min-h-screen flex-col items-center justify-between font-karla text-white">
      <div className={"flex flex-col justify-center align-middle gap-4 h-screen w-screen "}> 
        <AnimatePresence>
          {(brain.thinking || brain.awake) ? 
          <Thinking className="justify-center w-screen h-screen bg-[#0c0c4c]"> 
            <AnimatedText className="absolute bottom-10 left-0 w-full text-center h-min" text={text} />
            <AnimatedSound className="px-3 animate-pulse" sound={wave}/> 
          </Thinking> : 
          display.page == "Home" ? <p className="w-full text-center animate-pulse text-2xl text-sky-950">Sleeping...</p>
          : display.page == "Movies" ? <MoviePage movies={display.props} /> 
          : <p className="w-full text-center animate-pulse text-2xl text-sky-950">Sleeping...</p>
          }
        </AnimatePresence>
        {error && <div className="absolute h-1/2 w-full">
          <p className="m-auto text-center">{error}</p>
        </div>}
      </div>
    </main>
  );
}
