package io.substrate.regdemo

import android.os.Handler
import android.os.Looper
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp

@Composable
fun T9WaitScreen() {
    // TD-151: mirror the XML T9Activity behavior — Handler.postDelayed on the
    // main looper. The previous LaunchedEffect+kotlinx.coroutines.delay path
    // depended on Compose IdleResource frame scheduling that the agent's idle
    // resource sometimes declares "idle" before the delay completes, masking
    // the post-delay recomposition from the walker. Plain Handler.postDelayed
    // runs on the same main looper the walker probes via
    // AccessibilityNodeInfo, so state mutation lands deterministically.
    //
    // Keep TD-56 invariant: always-mounted Text whose String state toggles
    // — conditional mount (`if (revealed) Text(...)`) doesn't always surface
    // in /semantic until something else forces an a11y refresh.
    var label by remember { mutableStateOf("") }
    Column(
        modifier = Modifier.padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp),
    ) {
        Text("T9 Wait")
        Button(onClick = {
            label = ""
            Handler(Looper.getMainLooper()).postDelayed({
                label = "T9 Delayed Element"
            }, 3000)
        }) {
            Text("T9 Trigger")
        }
        Text(
            label,
            modifier = Modifier.semantics(mergeDescendants = false) {
                contentDescription = label.ifEmpty { "t9-placeholder" }
            },
        )
    }
}
