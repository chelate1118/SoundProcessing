package sound.ui.filter

import processing.core.PVector
import sound.Processing
import sound.rust.Rust
import sound.ui.*
import kotlin.math.PI
import kotlin.math.pow
import kotlin.math.sqrt

class FilterPoint(
    sketch: Processing, position: PVector
): Component(sketch, Order.Priority.High) {
    companion object {
        fun new(sketch: Processing): FilterPoint {
            sketch.let {
                return FilterPoint(it, PVector(it.width / 2F, it.height / 2F))
            }
        }

        private var currentIndex = 0.toChar()
    }

    private val radius = 20F
    private val style = Style(sketch).apply {
        strokeWeight = 3F
        fillColor = sketch.color(121, 224, 238)
    }

    private val index = currentIndex++

    var position = position
        private set

    private val frequencyRust
        get() = (position.x / sketch.width * 255F).coerceIn(0F, 254F) + 1

    private val qualityRust
        get() = ((1 - position.y / sketch.height) * 255F).coerceIn(0F, 254F) + 1

    private val message: String
        get() {
            val frequencyChar = frequencyRust.toInt().toChar()
            val qualityChar = qualityRust.toInt().toChar()

            return "$frequencyChar$qualityChar"
        }

    fun amplitudeFromX(x: Float): Float {
        val xRust = (x / sketch.width * 255F).coerceIn(0F, 254F) + 1
        val frequency = 1.03.pow(xRust.toDouble() / 2 + 150)
        val aimedFrequency = 1.03.pow(frequencyRust.toDouble() / 2 + 150)
        val quality = (qualityRust - 128.0) / 180.0 + 1.0
        val highPassFrequency = aimedFrequency / quality
        val lowPassFrequency = aimedFrequency * quality
        val highPassFrequencyRC = 0.5 / PI / highPassFrequency
        val lowPassFrequencyRC = 0.5 / PI / lowPassFrequency
        val highPass2PiFRC = 2.0 * PI * frequency * highPassFrequencyRC
        val lowPass2PiFRC = 2.0 * PI * frequency * lowPassFrequencyRC

        val highPassAmplitude = 1 / sqrt(1 + (1 / highPass2PiFRC).pow(2))
        val lowPassAmplitude = 1 / lowPass2PiFRC / sqrt(1 + (1 / lowPass2PiFRC).pow(2))

        return (highPassAmplitude * lowPassAmplitude).toFloat()
    }

    override fun onCreated() {
        Filter.list.add(this)

        Rust.sendMessage(message, 'a')
    }

    override fun update(): State {
        return State.Alive
    }

    override fun display() {
        style.apply()
        sketch.ellipse(position.x, position.y, radius*2, radius*2)
    }

    override fun isMouseIn(mousePosition: PVector): Boolean {
        return PVector.dist(position, mousePosition) < radius
    }

    override fun mouseOut() {
        style.apply {
            strokeColor = sketch.color(0)
        }
    }

    override fun mouseIn() {
        style.apply {
            strokeColor = sketch.color(31, 31, 255)
        }
    }

    override fun mousePressed() {
        position = sketch.mousePosition

        Rust.sendMessage("$index$message", 'm')
    }

    override fun compareTo(other: Component): Int {
        if (other is FilterPoint) {
            val comparePositionX = position.x.compareTo(other.position.x)

            if (comparePositionX == 0) {
                return super.compareTo(other)
            }

            return comparePositionX
        }

        return super.compareTo(other)
    }
}