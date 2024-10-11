package com.example.objectionapp

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.Text
import androidx.compose.ui.Alignment
import androidx.compose.ui.draw.clip
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import coil.compose.AsyncImage
import androidx.compose.animation.AnimatedVisibilityScope
import androidx.compose.animation.ExperimentalSharedTransitionApi
import androidx.compose.animation.SharedTransitionScope
import androidx.compose.animation.core.tween
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.material3.LocalContentColor
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.Dp

@OptIn(ExperimentalSharedTransitionApi::class)
@Composable
fun SharedTransitionScope.ContentView(
    content: Content,
    padding: PaddingValues,
    animatedVisibilityScope: AnimatedVisibilityScope?
) {
    when (content) {
        is Content.ParagraphContent -> {
            ParagraphView(content.def, padding)
        }

        is Content.HeadlineContent -> {
            HeadlineView(content.def, padding)
        }

        is Content.QuoteContent -> {
            Quote(content.def, padding)
        }

        is Content.ObjectPreviewContent -> {
            ObjectPreviewView(content.def, padding, animatedVisibilityScope = animatedVisibilityScope)
        }

        is Content.CallToActionContent -> {
            CallToActionView(content.def, padding)
        }

        is Content.ObjectGroupContent -> {
            ObjectGroupView(content.def, padding, animatedVisibilityScope)
        }
    }
}

@Composable
fun ParagraphView(content: Paragraph, padding: PaddingValues) {
    Box(Modifier.padding(padding)) {
        Text(content.text, color = content.color?.intoColor() ?: LocalContentColor.current, fontSize = 16.sp)
    }
}

@Composable
fun HeadlineView(content: Headline, padding: PaddingValues) {
    Box(
        Modifier
            .padding(padding)
            .padding(top = 20.dp)) {
        Text(
            content.text,
            color = content.color?.intoColor() ?: LocalContentColor.current,
            fontSize = 30.sp,
            fontWeight = FontWeight.Bold
        )
    }
}

@Composable
fun Quote(content: Quote, padding: PaddingValues) {
// TODO: fixme. I should work with the material theme
//    Box(modifier = Modifier.padding(padding)) {
//        Column(
//            modifier = Modifier
//                .background(surface.value.backgroundColor1.intoColor())
//                .clip(RoundedCornerShape(10))
//        ) {
//            Box(modifier = Modifier.padding(16.dp)) {
//                Text(content.text, color = surface.value.backgroundColor1.intoColor())
//            }
//            Box(
//                modifier = Modifier
//                    .padding(16.dp)
//                    .background(attributionSurface.value.backgroundColor1.intoColor())
//            ) {
//                Text(content.attribution)
//            }
//        }
//    }
}

@Composable
fun CallToActionView(content: CallToAction, padding: PaddingValues) {
    val navController = useNavController()

    Column(
        modifier = Modifier
            .padding(padding)
            .fillMaxWidth(),
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Button(
            onClick = { navController.navigate(encodeObjectIdIntoPageRoute(content.targetObject)) },
//            colors = ButtonDefaults.buttonColors(
//                containerColor = surface.value.backgroundColor1.intoColor(),
//                contentColor = surface.value.foregroundColor1.intoColor(),
//                disabledContainerColor = surface.value.backgroundColor3.intoColor(),
//                disabledContentColor = surface.value.backgroundColor3.intoColor(),
//            ),
            content = {
                Row(
                    horizontalArrangement = Arrangement.spacedBy(10.dp),
                    verticalAlignment = Alignment.CenterVertically,
                    modifier = Modifier.padding(8.dp)
                ) {
                    content.icon?.let {
                        StandardIcon(it)
                    }

                    Text(content.title, fontSize = 16.sp)
                }
            }
        )
    }
}

@OptIn(ExperimentalSharedTransitionApi::class)
@Composable
fun SharedTransitionScope.ObjectPreviewView(
    content: ObjectPreview,
    padding: PaddingValues,
    width: Dp? = null,
    animatedVisibilityScope: AnimatedVisibilityScope? = null
) {
    val obj = usePage(content.objectId)
    val navController = useNavController()

    Box(Modifier.padding(padding)) {
        Card(
            modifier = if (width != null) {
                Modifier.width(width)
            } else {
                Modifier
            },
            onClick = {
                navController.navigate(route = encodeObjectIdIntoPageRoute(content.objectId))
            },
//            colors = CardDefaults.cardColors(
//                containerColor = ,
//                contentColor = surface.value.foregroundColor1.intoColor(),
//                disabledContentColor = surface.value.foregroundColor3.intoColor(),
//                disabledContainerColor = surface.value.backgroundColor2.intoColor()
//            )
        ) {
            obj?.imageUrls?.first()?.let {
                AsyncImage(
                    model = it,
                    contentDescription = "An image",
                    clipToBounds = true,
                    contentScale = ContentScale.Crop,
                    modifier = if (animatedVisibilityScope != null) {
                        Modifier
                            .sharedElement(
                                state = rememberSharedContentState("${content.objectId}/image"),
                                animatedVisibilityScope = animatedVisibilityScope,
                                boundsTransform = { _, _ ->
                                    tween(durationMillis = 300)
                                }
                            )
                    } else {
                        Modifier
                    }
                        .height(200.dp)
                        .fillMaxWidth()
                        .clip(RoundedCornerShape(12.dp)),
                    onState = { state ->
                        println(state)
                    }
                )
            }

            Column(
                Modifier.padding(horizontal = 10.dp, vertical = 12.dp),
                verticalArrangement = Arrangement.spacedBy(10.dp)
            ) {
                Text(
                    "${obj?.title}",
//                    color = surface.value.foregroundColor1.intoColor(),
                    fontSize = 16.sp,
                    fontWeight = FontWeight.Bold,
                )

                obj?.content?.find { it is Content.ParagraphContent }?.let {
                    Text(
                        (it as Content.ParagraphContent).def.text,
//                        color = surface.value.foregroundColor3.intoColor(),
                        maxLines = 2,
                        overflow = TextOverflow.Ellipsis
                    )
                }
            }
        }
    }
}

@OptIn(ExperimentalSharedTransitionApi::class)
@Composable
fun SharedTransitionScope.ObjectGroupView(
    content: ObjectGroup,
    padding: PaddingValues,
    animatedVisibilityScope: AnimatedVisibilityScope?
) {
    Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
        Text(
            content.title,
            fontSize = 20.sp,
//            color = surface.value.foregroundColor1.intoColor(),
            modifier = Modifier.padding(padding)
        )

        LazyRow(horizontalArrangement = Arrangement.spacedBy(8.dp), contentPadding = padding) {
            for (objectId in content.objects) {
                item {
                    ObjectPreviewView(
                        ObjectPreview(objectId, null, null),
                        padding = PaddingValues(0.dp),
                        width = 200.dp,
                        animatedVisibilityScope = animatedVisibilityScope
                    )
                }
            }
        }
    }
}
