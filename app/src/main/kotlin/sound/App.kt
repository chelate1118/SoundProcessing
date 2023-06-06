/*
 * This Kotlin source file was generated by the Gradle 'init' task.
 */
package sound

import processing.core.*
import sound.ui.*

// Main
class Processing: AdvancedPApplet() {
    val ui = UI(this)

    override fun settings() {
        size(800, 800)
    }
    
    override fun setup() {
        ui.setup()
    }
    
    override fun draw() {
        ui.draw()
    }

    override fun mousePressed() {
        ui.mousePressed()
    }

    override fun mouseReleased() {
        ui.mouseReleased()
    }
}

fun main() {
    PApplet.main("sound.Processing")
}
