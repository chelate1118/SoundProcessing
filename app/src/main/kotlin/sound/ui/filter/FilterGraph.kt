package sound.ui.filter

import processing.core.PVector
import sound.Processing
import sound.ui.Component
import sound.ui.Order
import sound.ui.State
import sound.ui.Style

class FilterGraph(sketch: Processing): Component(sketch, Order.Priority.Low) {
    private val style = Style(sketch).apply {
        fill = false
    }

    override fun update(): State {
        Filter.list.sort()
        
        return State.Alive
    } 

    override fun display() {
        style.apply()
        
        with (sketch) {
            for (filterPoint in Filter.list) {
                beginShape()

                for (x in 0 until width) {
                    val xFloat = x.toFloat()
                    var yFloat = filterPoint.amplitudeFromX(xFloat) * height
                    yFloat = (height - yFloat)

                    vertex(xFloat, yFloat)
                }

                endShape()
            }
        }
    }

    override fun isMouseIn(mousePosition: PVector): Boolean = false
}