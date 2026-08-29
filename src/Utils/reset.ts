import React from "react";
import { NavigateFunction } from "react-router-dom";
import { Options } from "../types";
import { resetSession } from "../Services/commands";

export default function reset(
    setOptions: React.Dispatch<React.SetStateAction<Options>>,
    navigate: NavigateFunction
): void {
    setOptions({
        copies: null,
        digital: false,
        filter: null,
        layout: null
    })

    void resetSession();

    navigate('/', { replace: true })    
}