package io.substrate.regdemo

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp

@Composable
fun T13LoginScreen() {
    val state by T13Store.state
    val errorMessage by T13Store.errorMessage
    val primary = when (state) {
        T13State.Locked, T13State.Error -> "T13 Locked"
        T13State.Unlocked -> "T13 Unlocked"
    }
    Column(
        modifier = Modifier.padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp),
    ) {
        Text("T13 Login")
        // TD-150: keep primary status as its own semantic node so the walker
        // picks up the recomposed contentDescription (was previously
        // merged with the adjacent header — the Locked→Unlocked transition
        // landed in state but didn't surface in /semantic until next tap).
        Text(
            primary,
            modifier = Modifier.semantics(mergeDescendants = false) {
                contentDescription = primary
            },
        )
        errorMessage?.let { msg ->
            Text(
                msg,
                modifier = Modifier.semantics(mergeDescendants = false) {
                    contentDescription = msg
                },
            )
        }
        Button(onClick = { T13Store.reset() }) {
            Text("T13 Reset")
        }
    }
}
