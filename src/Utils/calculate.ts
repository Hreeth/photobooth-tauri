import { Config } from "../Contexts/DataContext"
import { Mode, Options } from "../types"

export default function calculate(
    options: Options,
    mode: Mode,
    config: Config
): number {
    let price = config.plans.find(_ => _.copies == options.copies)?.price ?? 0
    let digitalPrice = options.digital && mode == Mode.AUTOMATIC ? 99 : 0

    price += digitalPrice

    return price ? price * 100 : 0
}