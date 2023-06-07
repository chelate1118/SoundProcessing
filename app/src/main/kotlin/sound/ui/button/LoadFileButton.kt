package sound.ui.button

import processing.core.PVector
import sound.Processing

class LoadFileButton(sketch: Processing): Button(sketch) {
    override val position = PVector(sketch.width - buttonWidth * 2.5F, buttonHeight)

    override fun display() {
        super.display()
    }

    override fun mouseClicked() {
        sketch.selectInput("Select audio file:", "fileSelected")
    }
}