import { event } from "@tauri-apps/api";
import { useEffect, useState } from "react";

interface Command {

}
interface CommandProps{
    Commands: Command[];
}


export class App{
    constructor(props: CommandProps){
        
    }
    const requestAsync = async (command: string) => {

    }
    useEffect(() => {
        props.Commands.map((command) => {
            event.listen(command.name, (event: {payload: string}) => {

            })
        })
        event.listen("speech", (event: {payload: string}) => {
          setTokens([event.payload]);
        });
      }, [])
}