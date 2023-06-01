package sound.ui

import sound.Processing

enum class State {
    Kill,
    Alive
}

class Order(val priority: Priority): Comparable<Order> {
    val id = currentId++
    
    enum class Priority {
        Background,
        Low,
        Top
    }
    
    override fun compareTo(other: Order): Int {
        priority.compareTo(other.priority).let {
            if (it != 0) return it
        }
        
        return id.compareTo(other.id)
    }
    
    companion object {
        var currentId = 0
    }
}

abstract class Component(val sketch: Processing) {
    abstract fun onCreated()
    abstract fun update(): State
    abstract fun display()
    fun onDestroy() {}
    fun isMouseIn(): Boolean = false
    fun mouseIn() {}
    fun mouseClick() {}
}