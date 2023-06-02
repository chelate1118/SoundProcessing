package sound

import processing.core.*

open class AdvancedPApplet: PApplet() {
    val mousePosition
        get() = PVector(mouseX.toFloat(), mouseY.toFloat())
}