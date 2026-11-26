package voorbeeld

// ANCHOR: begroeting
fun begroeting(naam: String): String = "Hallo, $naam"
// ANCHOR_END: begroeting

fun main() {
    println(begroeting("Firda"))
}
