import { Options } from "../Contexts/DataContext";

export default function calculate(
    options: Options,
    plans: Array<{
        strips: number,
        title: string,
        price: number,
        popular: boolean
    }>
): number {
    let price = plans.find(_ => _.strips == options.copies)?.price
    if (options.digital && price) price += 99

    return price ? price * 100 : 0
}