package sound.ui.filter

import processing.core.PVector
import sound.Processing
import sound.ui.*

class FilterPoint(sketch: Processing, private var position: PVector): Component(sketch, Order.Priority.Top) {
    val radius = 20F
    val style = Style(sketch).apply {
        strokeWeight = 3F
        fillColor = sketch.color(121, 224, 238)
    }

    override fun update(): State {
        return State.Alive
    }

    override fun display() {
        style.apply()
        sketch.ellipse(position.x, position.y, radius*2, radius*2)
    }

    override fun isMouseIn(mousePosition: PVector): Boolean {
        return PVector.dist(position, mousePosition) < radius
    }

}