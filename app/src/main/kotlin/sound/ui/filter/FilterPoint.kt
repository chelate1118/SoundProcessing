package sound.ui.filter

import processing.core.PVector
import sound.Processing
import sound.ui.*

class FilterPoint(
    sketch: Processing, position: PVector
): Component(sketch, Order.Priority.High) {
    private val radius = 20F
    private val style = Style(sketch).apply {
        strokeWeight = 3F
        fillColor = sketch.color(121, 224, 238)
    }

    var position = position
        private set

    override fun onCreated() {
        Filter.list.add(this)
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

    override fun mouseOut() {
        style.apply {
            strokeColor = sketch.color(0)
        }
    }

    override fun mouseIn() {
        style.apply {
            strokeColor = sketch.color(31, 31, 255)
        }
    }

    override fun mousePressed() {
        position = sketch.mousePosition
    }

    override fun compareTo(other: Component): Int {
        if (other is FilterPoint) {
            return position.x.compareTo(other.position.x)
        }

        return super.compareTo(other)
    }
}