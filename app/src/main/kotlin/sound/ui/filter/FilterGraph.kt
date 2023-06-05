package sound.ui.filter

import processing.core.PVector
import sound.Processing
import sound.ui.Component
import sound.ui.Order
import sound.ui.State
import sound.ui.Style

class FilterGraph(sketch: Processing): Component(sketch, Order.Priority.Low) {
    override fun update(): State {
        Filter.list.sort()
        
        return State.Alive
    } 

    override fun display() {
        Style(sketch).apply()
        
        for (i in 1 until Filter.list.size) {
            sketch.line(Filter.list[i].position, Filter.list[i-1].position)
        }
    }

    override fun isMouseIn(mousePosition: PVector): Boolean = false
}