'use client';

import Image from "next/image";
import { event, invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import { AnimatePresence, delay, motion } from "framer-motion";
import { MovieResults, MovieSearch } from "./models/movieModels";
import MoviePage from "./pages/movies";
import UseBrain from "./brain/useBrain";
import AnimatedText from "@/components/animatedText";
import Thinking from "@/components/thinking";

export default function Home() {
  const [text, setText] = useState<string>("hello world");
  const [page, setPage] = useState<"Movies"|"Home">("Home")
  const [pageProps, setPageProps] = useState<any>();

  const [display, setDisplay] = useState<{
    page: "Movies" | "Home",
    props: Record<string, any>
  }>({
    page: "Home",
    props: {},
  });

  const [sentence, setSentence] = useState<string>("show me a superman movie");
  const [error, setError] = useState<string>("");

  const brain = UseBrain({
      // onThought: (thought) => setText(thought),
      onMovieList: (movies) => {
        setDisplay({...display, props: movies, page: "Movies"})
      },
      onError: (error) => setError(error),
      onSpeech: (speech) => setText(speech),

      onSelect: (choice) => {},
  })

  return (
    <main className="flex min-h-screen flex-col items-center justify-between bg-sky-950 font-karla text-white">
      <div className={"flex flex-col justify-center align-middle gap-4 h-screen w-screen"}>
        {brain.thinking && <Thinking> <AnimatedText text={text} /> </Thinking> }
        {display.page == "Movies" && <MoviePage movies={pageProps} />}
        {!brain.thinking && <div className="absolute bottom-5 w-full flex justify-center">
          <input value={sentence} onChange={(event) => setSentence(event.target.value)} className="border bg-black bg-opacity-50 px-9 py-2 rounded-full" />
          <button className="border m-auto px-9 py-2 bg-black bg-opacity-50 rounded-lg " onClick={() => {
            
            invoke("speak", {command: sentence}).then((response) => console.log(response))
          }}>speak</button>
        </div>}
        <div className="absolute h-1/2 w-full">
          <p className="m-auto text-center">{error}</p>
        </div>
      </div>
    </main>
  );
}
