import React from "react";
import { Options } from "../Contexts/DataContext";
import { NavigateFunction } from "react-router-dom";

export default function reset(
    setOptions: React.Dispatch<React.SetStateAction<Options>>,
    setImages: React.Dispatch<React.SetStateAction<Array<string>>>,
    navigate: NavigateFunction

): void {
    setOptions({
        copies: null,
        digital: false,
        print: null
    })

    setImages([])

    navigate('/', { replace: true })    
}