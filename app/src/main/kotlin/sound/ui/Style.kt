package sound.ui

import sound.Processing

class Style(val sketch: Processing) {
    var fill: Boolean = true
    var fillColor: Int = 0
    var stroke: Boolean = true
    var strokeWeight: Float = 2F
    var strokeColor: Int = 0
    
    fun apply() {
        applyStroke()
        applyFill()
    }
    
    private fun applyStroke() {
        with (sketch) {
            if (stroke) {
                strokeWeight(strokeWeight)
                stroke(strokeColor)
            } else {
                noStroke()
            }
        }
    }
    
    private fun applyFill() {
        with (sketch) {
            if (fill) {
                fill(fillColor)
            } else {
                noFill()
            }
        }
    }
}