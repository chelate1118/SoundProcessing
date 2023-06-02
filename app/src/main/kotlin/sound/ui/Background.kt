package sound.ui

import processing.core.PVector
import sound.Processing
import sound.ui.filter.FilterPoint

class Background(sketch: Processing): Component(sketch, Order.Priority.Background) {
    private val color = sketch.color(152, 238, 204)
    
    override fun update(): State = State.Alive

    override fun display() {
        sketch.background(color)
    }
    
    override fun isMouseIn(position: PVector): Boolean = true

    override fun mouseClicked() {
        with (sketch) {
            ui.createComponent(FilterPoint(this, mousePosition))
        }
    }
}