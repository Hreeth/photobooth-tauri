import React from "react";
import { NavigateFunction } from "react-router-dom";
import { Options } from "../types";

export default function reset(
    setOptions: React.Dispatch<React.SetStateAction<Options>>,
    setImages: React.Dispatch<React.SetStateAction<Array<string>>>,
    navigate: NavigateFunction

): void {
    setOptions({
        copies: null,
        digital: false,
        print: null,
        layout: null
    })

    setImages([])

    navigate('/', { replace: true })    
}