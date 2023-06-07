package sound.ui.button

import processing.core.PVector
import sound.Processing
import sound.ui.filter.FilterPoint

class AddFilterPointButton(sketch: Processing): Button(sketch) {
    override val position = PVector(sketch.width - buttonWidth, buttonHeight)
    
    override fun display() {
        super.display()
    }
    
    override fun mouseClicked() {
        sketch.ui.createComponent(FilterPoint.new(sketch))
    }
}