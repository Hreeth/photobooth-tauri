import { Options, Plan } from "../Contexts/DataContext";

export default function calculate(
    options: Options,
    plans: Array<Plan>
): number {
    let price = plans.find(_ => _.copies == options.copies)?.price
    if (options.digital && price) price += 99

    return price ? price * 100 : 0
}