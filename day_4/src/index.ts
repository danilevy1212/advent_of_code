import { err, Result, ok, combine } from "neverthrow"
import { readFile } from "fs"

// The expected fields of a Passport
interface PassPort {
    byr: string // (Birth Year)
    iyr: string // (Issue Year)
    eyr: string // (Expiration Year)
    hgt: string // (Height)
    hcl: string // (Hair Color)
    ecl: string // (Eye Color)
    pid: string // (Passport ID)
    cid?: string // (Country ID)
}

const MUST_FIELDS: readonly string[] = Object.freeze([
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid",
])

const VALID_ECL: readonly string[] = Object.freeze([
    "amb",
    "blu",
    "brn",
    "gry",
    "grn",
    "hzl",
    "oth",
])

readFile(`${process.cwd()}/input.txt`, { encoding: "utf-8" }, (err, data) => {
    if (err) {
        throw err
    }

    const inputText = data.split(/^\s*\n/gm).map((x) => x.replace(/\s+/g, " ").trim())

    const maybePassports = combine(inputText.map(toPassport).filter(x => x.isOk()))

    if (maybePassports.isOk())
        console.log(`Valid Passports: ${maybePassports.value.length}`)

})

function toPassport(rawRecord: string): Result<PassPort, Error> {
    return combine<unknown, Error>(
        rawRecord.split(" ").map(x => {
            const [key, value] = x.split(":")

            if (typeof key !== 'string' && typeof value !== 'string')
                return err(new Error(`Can't understand line: ${rawRecord}`))

            return ok({
                [key]: value
            } as object)
        }))
        .andThen((keyVals) => ok<unknown, Error>(
            keyVals.reduce((acc, elt) => ({
                ...(elt as unknown as object),
                ...(acc as unknown as object),
            }), {} as object)))
        // PART 1
        .andThen((potentialPass) => {
            const keys = Object.keys(potentialPass as object)

            if (keys.length > 8)
                return err(new Error(`${rawRecord} contains to many fields.`))

            const hasKeys = keys
                .map(x => MUST_FIELDS.includes(x))
                .filter(x => x)

            if(hasKeys.length !== MUST_FIELDS.length)
                return err(new Error(`${rawRecord} not all necessary fields present.`))

            if(keys.length === 8 && !(potentialPass as PassPort)["cid"])
                return err(new Error(`${rawRecord} unrecognized optional field`))

            return ok({
                ...(potentialPass as object)
            } as PassPort)
        }) // PART 2
        .andThen((t) => {
            const uncheckedPass = { ...(t as object) } as PassPort
            const {
                byr,
                iyr,
                eyr,
                hgt,
                hcl,
                ecl,
                pid
            } = uncheckedPass

            const [nByr, nIyr, nEyr] = [byr, iyr, eyr].map(Number)

            // (Birth Year) - four digits; at least 1920 and at most 2002.
            if (!(nByr >= 1920 && nByr <= 2002))
                return err(new Error(`Invalid byr ${byr}`))

            // Issue Year) - four digits; at least 2010 and at most 2020.
            if (!(nIyr >= 2010 && nIyr <= 2020))
                return err(new Error(`Invalid iyr ${iyr}`))

            // (Expiration Year) - four digits; at least 2020 and at most 2030.
            if (!(nEyr >= 2020 && nEyr <= 2030))
                return err(new Error(`Invalid eyr ${eyr}`))

            if (!validateHgt(hgt))
                return err(new Error(`Invalid hgt ${hgt}`))

            if (!validateHcl(hcl))
                return err(new Error(`Invalid hcl ${hcl}`))

            if (!VALID_ECL.includes(ecl))
                return err(new Error(`Invalid ecl ${ecl}`))

            if (!(pid.length === 9 && pid.replace(/[0-9]/g, "").length === 0))
                return err(new Error(`Invalid pid ${pid}`))

            return ok(uncheckedPass)
        })
}

function validateHgt(hgt: string): boolean {
    const [isCm, isIn] = ['cm', 'in'].map(x => hgt.endsWith(x))

    if (isCm) {
        const height = Number(hgt.replace('cm', ''))

        return height >= 150 && height <= 193
    }


    if (isIn) {
        const height = Number(hgt.replace('in', ''))

        return height >= 59 && height <= 76
    }

    return false
}

function validateHcl(hcl: string): boolean {
    return hcl.startsWith("#") &&
        hcl.length === 7 &&
        hcl.slice(1).replace(/[a-f0-9]/g, "").length === 0
}
