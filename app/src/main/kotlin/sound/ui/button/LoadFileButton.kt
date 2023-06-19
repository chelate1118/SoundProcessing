package sound.ui.button

import processing.core.PImage
import processing.core.PVector
import sound.Processing

class LoadFileButton(sketch: Processing): Button(sketch) {
    companion object {
        private const val IMAGE_WIDTH = 25F
        private const val IMAGE_HEIGHT = 25F
    }

    override val position = PVector(sketch.width - buttonWidth * 2.5F, buttonHeight)
    private val loadFileImage: PImage = sketch.loadImage("file_open.png")

    override fun display() {
        super.display()

        with (sketch) {
            pushMatrix()
            translate(position.x, position.y)
            image(loadFileImage, -IMAGE_WIDTH/2F, -IMAGE_HEIGHT/2F, IMAGE_WIDTH, IMAGE_HEIGHT)
            popMatrix()
        }
    }

    override fun mouseClicked() {
        sketch.selectInput("Select audio file:", "fileSelected")
    }
}