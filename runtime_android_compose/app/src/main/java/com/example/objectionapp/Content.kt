package com.example.objectionapp

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonColors
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp

@Composable
fun ContentView(content: Content, modifier: Modifier) {
    when (content) {
        is Content.ParagraphContent -> {
            ParagraphView(content.def, modifier)
        }
        is Content.QuoteContent -> {
            Quote(content.def, modifier)
        }
        is Content.ObjectPreviewContent -> {

        }
        is Content.CallToActionContent -> {
            CallToActionView(content.def, modifier)
        }
        is Content.ObjectGroupContent -> {

        }
        else -> {
            println("Unreachable")
        }
    }
}

@Composable
fun ParagraphView(content: Paragraph, modifier: Modifier) {
    val surface = useSurface()

    Box(modifier) {
        Text(content.text, color = surface.value.foregroundColor2.intoColor())
    }
}

@Composable
fun Quote(content: Quote, modifier: Modifier) {
    val surface = useSurface(content.surface)
    val attributionSurface = useSurface(content.attributionSurface)

    Box(modifier = modifier) {
        Column(
            modifier = Modifier
                .background(surface.value.backgroundColor1.intoColor())
                .clip(RoundedCornerShape(10))
        ) {
            Box(modifier = Modifier.padding(16.dp)) {
                Text(content.text, color = surface.value.backgroundColor1.intoColor())
            }
            Box(
                modifier = Modifier
                    .padding(16.dp)
                    .background(attributionSurface.value.backgroundColor1.intoColor())
            ) {
                Text(content.attribution)
            }
        }
    }
}

@Composable
fun CallToActionView(content: CallToAction, modifier: Modifier) {
    val navController = useNavController()
    val surface = useSurface(content.surface)

    Column(modifier = modifier.fillMaxWidth(), horizontalAlignment = Alignment.CenterHorizontally) {
        Button(
            onClick = { navController.navigate(encodeObjectIdIntoPageRoute(content.targetObject)) },
            colors = ButtonColors(
                containerColor = surface.value.backgroundColor1.intoColor(),
                contentColor = surface.value.foregroundColor1.intoColor(),
                disabledContainerColor = surface.value.backgroundColor3.intoColor(),
                disabledContentColor = surface.value.backgroundColor3.intoColor(),
            ),
            content = {
                Row(
                    horizontalArrangement = Arrangement.spacedBy(10.dp),
                    verticalAlignment = Alignment.CenterVertically,
                    modifier = Modifier.padding(10.dp)
                ) {
                    content.icon?.let {
                        StandardIcon(it)
                    }

                    Text(content.title, fontSize = 20.sp)
                }
            }
        )
    }
}
