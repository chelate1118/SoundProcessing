package sound

import processing.core.*

open class AdvancedPApplet: PApplet() {
    val mousePosition
        get() = PVector(mouseX.toFloat(), mouseY.toFloat())

    fun line(vec1: PVector, vec2: PVector) {
        line(vec1.x, vec1.y, vec2.x, vec2.y)
    }
}