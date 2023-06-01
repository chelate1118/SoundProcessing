package sound.ui.filter

import processing.core.PVector
import sound.Processing

class Filter(private val sketch: Processing) {
    lateinit var position: PVector
    lateinit var text: String

    fun display() {
        with (sketch) {
            ellipse(position.x, position.y, width/4F, height/4F)
            text(text, 10F, 10F)
        }
    }
}