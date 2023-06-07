package sound.ui.button

import processing.core.PVector
import sound.Processing
import sound.ui.Component
import sound.ui.Order
import sound.ui.State
import sound.ui.Style
import kotlin.math.abs

abstract class Button(sketch: Processing): Component(sketch, Order.Priority.Top) {
    protected open val buttonWidth = 50F
    protected open val buttonHeight = 50F
    protected open val buttonCornerRadii = 15F
    protected abstract val position: PVector
    protected open val style = Style(sketch).apply {
        strokeColor = sketch.color(150)
        fillColor = sketch.color(200)
    }

    override fun update() = State.Alive

    override fun display() {
        style.apply()
        with (sketch) {
            pushMatrix()
            translate(position.x, position.y)
            rect(
                -buttonWidth/2F,
                -buttonHeight/2F,
                buttonWidth,
                buttonHeight,
                buttonCornerRadii
            )
            popMatrix()
        }
    }

    override fun isMouseIn(mousePosition: PVector): Boolean {
        val sub = PVector.sub(position, mousePosition)

        return (abs(sub.x) < buttonWidth / 2F && abs(sub.y) < buttonHeight / 2F)
    }

    override fun mouseIn() {
        style.apply {
            strokeWeight = 3F
            strokeColor = sketch.color(0, 0, 255)
            fillColor = sketch.color(180)
        }
    }

    override fun mouseOut() {
        style.apply {
            strokeWeight = 3F
            strokeColor = sketch.color(150)
            fillColor = sketch.color(200)
        }
    }
}