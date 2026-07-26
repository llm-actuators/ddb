package io.substrate.regdemo

import androidx.compose.runtime.mutableStateOf

enum class T13State { Locked, Unlocked, Error }

object T13Store {
    val state = mutableStateOf(T13State.Locked)

    // TD-149: surface the last error message as its own observable state so
    // T13LoginScreen renders it independent of the lifecycle of the Error
    // state. Previously the screen rendered "Invalid credentials" only while
    // state == Error; the walker can race the recomposition and miss it.
    val errorMessage = mutableStateOf<String?>(null)

    fun handle(email: String, password: String, completion: (Boolean, String?) -> Unit) {
        if (email == "t13@example.com" && password == "t13pass") {
            state.value = T13State.Unlocked
            errorMessage.value = null
            completion(true, null)
        } else {
            state.value = T13State.Error
            errorMessage.value = "Invalid credentials"
            completion(false, "Invalid credentials")
        }
    }

    fun reset() {
        state.value = T13State.Locked
        errorMessage.value = null
    }
}
