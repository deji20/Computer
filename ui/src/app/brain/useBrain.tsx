'use client';

import {useEffect, useState} from "react";
import { MovieSearch } from "../models/movieModels";
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';


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
        if(abilities.onThought) 
            events.push(listen("thought", (event: {payload: string}) => abilities.onThought?.(event.payload)))
        if(abilities.onSelect) 
            events.push(listen("select", (event: {payload: string}) => abilities.onSelect?.(event.payload)))
        if(abilities.onMovieList) 
            events.push(listen("movieList", (event: {payload: string}) => abilities.onMovieList?.(JSON.parse(event.payload))))
        if(abilities.onSpeech) 
            events.push(listen("speak", (event: {payload: string}) => abilities.onSpeech?.(event.payload)))
        if(abilities.onError) 
            events.push(listen("error", (event: {payload: string}) => abilities.onError?.(event.payload)))
        if(abilities.onListen)
            events.push(listen("audio", (event: {payload: number}) => abilities.onListen?.(event.payload)))

        events.push(listen<boolean>("loading", (event) => setThinking(event.payload)))
        events.push(listen<boolean>("awake", (event) => setAwake(true)));
        
        return () => {
            if(events.length){
                events.forEach(e => e())
            }
        }
    }, [])

    const ask = (thought: string) => invoke("speak", {command: thought}).then((response) => console.log(response))
    return {thinking, awake, ask} as Brain;
}