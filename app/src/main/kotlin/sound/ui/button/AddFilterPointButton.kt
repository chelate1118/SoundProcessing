package sound.ui.button

import processing.core.PImage
import processing.core.PVector
import sound.Processing
import sound.ui.filter.FilterPoint

class AddFilterPointButton(sketch: Processing): Button(sketch) {
    companion object {
        private const val IMAGE_WIDTH = 25F
        private const val IMAGE_HEIGHT = 25F
    }

    override val position = PVector(sketch.width - buttonWidth, buttonHeight)
    private val addImage: PImage = sketch.loadImage("add.png")

    override fun display() {
        super.display()

        with (sketch) {
            pushMatrix()
            translate(position.x, position.y)
            image(addImage, -IMAGE_WIDTH/2F, -IMAGE_HEIGHT/2F, IMAGE_WIDTH, IMAGE_HEIGHT)
            popMatrix()
        }
    }
    
    override fun mouseClicked() {
        sketch.ui.createComponent(FilterPoint.new(sketch))
    }
}