import {useEffect, useState} from "react";
import { MovieResults, MovieSearch } from "../models/movieModels";
import { event, invoke } from "@tauri-apps/api";

interface Abilities {
    onThought?: (thought: string) => void;
    onSelect?: (selected: string) => void;
    onMovieList?: (movies: MovieSearch[]) => void;
    onSpeech?: (speech: string) => void;
    onError?: (error: string) => void;
}

interface BrainProps extends Abilities {}

export default function UseBrain(abilities: BrainProps){
    const [thinking, setThinking] = useState<boolean>(false);

    useEffect(() => {
        if(abilities.onThought) event.listen("thought", (event: {payload: string}) => abilities.onThought?.(event.payload))
        if(abilities.onSelect) event.listen("select", (event: {payload: string}) => abilities.onSelect?.(event.payload))
        if(abilities.onMovieList) event.listen("movieList", (event: {payload: MovieSearch[]}) => abilities.onMovieList?.(event.payload))
        if(abilities.onSpeech) event.listen("speak", (event: {payload: string}) => abilities.onSpeech?.(event.payload))
        if(abilities.onError) event.listen("error", (event: {payload: string}) => abilities.onError?.(event.payload))

        event.listen<boolean>("loading", (event) => setThinking(event.payload))
    }, [])

    const ask = (thought: string) => invoke("speak", {command: thought}).then((response) => console.log(response))
    return {ask, thinking}
}