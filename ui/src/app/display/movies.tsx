'use client';

import { useState } from "react"
import { MovieResults, MovieSearch } from "../models/movieModels"
import Image from "next/image"

interface MoviePageProps {
    movies?: MovieResults
}

export default function MoviePage(props: MoviePageProps){
    const [movie, setMovie] = useState<MovieSearch>()
    return <div>
        <p>
            {props.movies?.results?.length}
            movies:
        </p>
        {movie ? <div>
            <iframe src={"https://vidsrc.xyz/embed/movie/" + movie.id} className="w-screen h-screen"></iframe>
            <div className="flex absolute bottom-0 gap-4 p-5 bg-black bg-opacity-20">
                <h2 className="m-auto">{movie.title}</h2>
                <p className="m-auto">{movie.overview}</p>
                <Image width={100} height={100} alt="movie poster" src={"https://image.tmdb.org/t/p/original" + movie.poster_path} className="w-24 h-24 border rounded-lg shadow-lg"/>
            </div>
        </div>
        :
        <ul className="flex overflow-x-scroll gap-5">
            {props.movies?.results?.map((movie, index) => 
            <div key={index} className="p-4 flex flex-col gap-2 w-96 h-96"> 
                <Image width={400} height={700} alt="movie poster" src={"https://image.tmdb.org/t/p/original" + movie.poster_path} className="w-full h-full border rounded-lg shadow-lg"/>
                <button className="px-3 py-1 border" onClick={() => setMovie(movie)}>{movie.title}</button>
            </div>
            )}
        </ul>}
    </div>
}