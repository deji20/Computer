'use client';
import { useEffect, useRef, useState } from "react";
import MoviePage from "./display/movies";
import UseBrain from "./brain/useBrain";
import AnimatedText from "../components/animatedText";
import Thinking from "../components/thinking";
import { AnimatePresence } from "framer-motion";
import { NextPage } from "next";
import { event, invoke } from "@tauri-apps/api";
import { motion } from "framer-motion";
import AnimatedSound from "@/components/animatedSound";

// invoke("start").then((response) => console.log(response))

export default function Home() {
  const [text, setText] = useState<string>("hello world");
  const [display, setDisplay] = useState<{page: "Movies" | "Home", props: any,}>({ page: "Home", props: {} });
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
    <main className="flex min-h-screen flex-col items-center justify-between bg-[#222223] font-karla text-white">
      <div className={"flex flex-col justify-center align-middle gap-4 h-screen w-screen bg-black bg-opacity-50 "}> 
        <AnimatePresence>
          {!(brain.thinking || brain.awake) ? 
          <Thinking className="border justify-center"> 
            <AnimatedText className="w-min m-auto mb-0 h-min" text={text} />
            <AnimatedSound className="" sound={wave}/> 
          </Thinking> : 
          <p className="w-full text-center animate-pulse text-2xl text-gray-900">Sleeping...</p>
          }
        </AnimatePresence>
        {display.page == "Movies" && <MoviePage movies={display.props} />}
        {error && <div className="absolute h-1/2 w-full">
          <p className="m-auto text-center">{error}</p>
        </div>}
      </div>
    </main>
  );
}
