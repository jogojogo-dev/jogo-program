import { sha256 } from "@noble/hashes/sha256";
import BN from "bn.js";

export class Fraction {
    numerator: BN;
    denominator: BN;

    constructor(numerator: BN, denominator: BN) {
        this.numerator = numerator;
        this.denominator = denominator
    }

    static fromNumber(numerator: number, denominator: number) {
        return new Fraction(new BN(numerator), new BN(denominator));
    }

    static fromJson(val: { numerator: BN, denominator: BN }) {
        return new Fraction(val.numerator, val.denominator);
    }

    mul(other: Fraction) {
        let numerator = this.numerator.mul(other.numerator);
        let denominator = this.denominator.mul(other.denominator);
        return new Fraction(numerator, denominator);
    }

    mulBN(other: BN) {
        return this.numerator.mul(other).div(this.denominator);
    }

    toJson() {
        return {
            numerator: this.numerator,
            denominator: this.denominator,
        };
    }

    toFloat() {
        return this.numerator.toNumber() / this.denominator.toNumber();
    }
}

export function randomSeed(lock: Uint8Array, lastRandomness: Uint8Array) {
    let hasher = sha256.create();
    hasher.update(lock);
    hasher.update(lastRandomness);
    return hasher.digest();
}

export function packBetMessage(
    bet: Uint8Array,
    point: Fraction,
) {
    let betMessage = new Uint8Array(48);
    betMessage.set(bet, 0);
    betMessage.set(point.numerator.toArray("le", 8), 32);
    betMessage.set(point.denominator.toArray("le", 8), 40);
    return betMessage;
}

export function computeCrashPoint(randomSig: Uint8Array, winRate: Fraction) {
    let hasher = sha256.create();
    hasher.update(randomSig);
    const hash = hasher.digest();
    const finalRand = Buffer.from(hash.slice(0, 4)).readUInt32LE();
    const scale = Fraction.fromNumber(Math.pow(2, 32), finalRand + 1);
    return scale.mul(winRate)
}