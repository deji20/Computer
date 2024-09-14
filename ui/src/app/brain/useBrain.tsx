'use client';

import {useEffect, useState} from "react";
import { MovieSearch } from "../models/movieModels";
import { event, invoke } from "@tauri-apps/api";

interface Abilities {
    onThought?: (thought: string) => void;
    onSelect?: (selected: string) => void;
    onMovieList?: (movies: MovieSearch[]) => void;
    onSpeech?: (speech: string) => void;
    onError?: (error: string) => void;
    onListen?: (audio: number) => void;
}

interface BrainProps extends Abilities {}


interface Brain {
    thinking: boolean;
    awake: boolean;
    ask: (thought: string) => void;
}

export default function UseBrain(abilities: BrainProps){
    const [thinking, setThinking] = useState<boolean>(false);
    const [awake, setAwake] = useState<boolean>(false);
    useEffect(() => {
        let events: any[] = [];
        if(event){
            if(abilities.onThought) 
                events.push(event.listen("thought", (event: {payload: string}) => abilities.onThought?.(event.payload)))
            if(abilities.onSelect) 
                events.push(event.listen("select", (event: {payload: string}) => abilities.onSelect?.(event.payload)))
            if(abilities.onMovieList) 
                events.push(event.listen("movieList", (event: {payload: string}) => abilities.onMovieList?.(JSON.parse(event.payload))))
            if(abilities.onSpeech) 
                events.push(event.listen("speak", (event: {payload: string}) => abilities.onSpeech?.(event.payload)))
            if(abilities.onError) 
                events.push(event.listen("error", (event: {payload: string}) => abilities.onError?.(event.payload)))
            if(abilities.onListen)
                events.push(event.listen("audio", (event: {payload: number}) => abilities.onListen?.(event.payload)))

            events.push(event.listen<boolean>("loading", (event) => setThinking(event.payload)))
            events.push(event.listen<boolean>("awake", (event) => { console.log("awake", event.payload); setAwake(true) }))
        }
        () => {
            if(events.length){
                events.forEach(e => e())
            }
        }
    }, [])

    const ask = (thought: string) => invoke("speak", {command: thought}).then((response) => console.log(response))
    return {thinking, awake, ask} as Brain;
}