package sound.ui.filter

import sound.rust.Rust

object Filter {
    val list = ArrayList<FilterPoint>()
    var currentAudio: String? = null
        set(value) {
            if (field != value) {
                Rust.sendMessage(value, 'p')
            }
            field = value
        }
}