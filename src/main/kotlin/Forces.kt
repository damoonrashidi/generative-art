import org.openrndr.application
import org.openrndr.color.ColorHSLa
import org.openrndr.color.ColorRGBa
import org.openrndr.draw.isolatedWithTarget
import org.openrndr.draw.renderTarget
import org.openrndr.extra.noclear.NoClear
import org.openrndr.extra.noise.Random
import org.openrndr.math.IntVector2
import org.openrndr.math.Vector2
import org.openrndr.shape.Circle
import org.openrndr.shape.Rectangle
import org.openrndr.shape.Triangle
import org.openrndr.shape.drawComposition
import kotlin.math.cos
import kotlin.math.sin
import kotlin.system.exitProcess

fun main() = application {
    configure {
        width = 32
        height = 32
        hideWindowDecorations = true
        position = IntVector2(0, 0)
    }

    program {
        extend(NoClear())
        extend {
            drawer.stroke = null
            val seed = System.currentTimeMillis().toString()
            Random.seed = seed
            val renderWidth = 8000
            val renderHeight = (8000 * 1.4).toInt()
            val bounds = Rectangle(0.0, 0.0, renderWidth.toDouble(), renderHeight.toDouble())
            val quads = CollisionDetection<Circle>(listOf(), bounds, 15)

            val zoom = Random.int(1_800, 3_200)
            val distort = Random.double(1.5, 4.2)
            val linePadding = 50.0
            val lineWidths: List<Double> = listOf(40.0, 50.0, 60.0)
            val allowEdgeOverflow = Random.bool(0.25)
            val allowHeavy = Random.bool(0.8)
            val allowChoppy = Random.bool(0.8)
            val backgroundColor = ColorHSLa(35.0, 0.13, 0.92).toRGBa()
            val palette = Palette.random()
            val minLineLength = 50.0
            val maxLineLength = renderHeight / 2.0
            val density = 3000

            println("seed: $seed")
            println("renderWidth: $renderWidth")
            println("renderHeight: $renderHeight")
            println("zoom: $zoom")
            println("distort: $distort")
            println("allowEdgeOverflow: $allowEdgeOverflow")
            println("allowChoppy: $allowChoppy")
            println("allowHeavy: $allowHeavy")
            println("palette: $palette")
            println("density: $density")
            val colorRegion: List<Pair<Triangle, ColorRGBa>> = generateSequence {
                Triangle(Random.point(bounds), Random.point(bounds), Random.point(bounds))
            }.take(10).toList().map {
                Pair(it, Random.pick(palette).toRGBa())
            }
            val canvas = renderTarget(renderWidth, renderHeight) { colorBuffer() }
            drawer.isolatedWithTarget(canvas) {
                ortho(canvas)

                drawer.fill = backgroundColor
                drawer.rectangle(0.0, 0.0, renderWidth.toDouble(), renderHeight.toDouble())

                val svg = drawComposition {

                    repeat(density) {
                        val isLong = Random.bool(0.005)
                        val lineRadius = run {
                            val heavy = Random.bool(0.1)
                            if (allowHeavy && heavy) Random.pick(lineWidths) * 4 else Random.pick(lineWidths)
                        }

                        val stepSize = run {
                            val choppy = Random.bool(0.1)
                            if (allowChoppy && choppy) bounds.width / 5000 else bounds.width / 10000
                        }
                        this.strokeWeight = lineRadius

                        var (x, y) = run {
                            if (allowEdgeOverflow && isLong) {
                                Random.point(bounds.scale(0.95))
                            } else Random.point(bounds.scale(0.8))
                        }
                        val linePoints = mutableListOf<Circle>()

                        drawer.fill = run {
                            val region = colorRegion.find { it.first.contains(Vector2(x, y)) }
                            region?.second ?: palette.first().toRGBa()
                        }

                        drawer.strokeWeight = lineRadius
                        drawer.stroke = drawer.fill

                        this.stroke = drawer.fill
                        this.strokeWeight = lineRadius

                        val innerBounds = if (allowEdgeOverflow && isLong) bounds.scale(0.95)
                        else bounds.scale(0.8)

                        while (Vector2(x, y) in innerBounds && !exceedsMaxLineLength(linePoints, maxLineLength)) {
                            val n = Random.simplex(x / zoom, y / zoom)
                            x += cos(n * distort) * lineRadius * stepSize
                            y += sin(n * distort) * lineRadius * stepSize
                            val neighbors = quads.getNeighbors(Circle(x, y, lineRadius))

                            if (neighbors.any {
                                    Vector2(x, y).distanceTo(it.center) < it.radius / 2 + lineRadius / 2 + linePadding
                                }) {
                                break
                            }

                            linePoints.add(Circle(x, y, lineRadius))
                        }

                        if (getLineLength(linePoints) > minLineLength) {
                            quads.addParticles(linePoints)
                            drawer.lineStrip(linePoints.map { it.center })
                            this.lineStrip(linePoints.map { it.center }, insert = true)
                        }
                    }
                }

                GenerativeArt.saveOutput("Forces",
                    canvas,
                    svg,
                    Vector2(renderWidth.toDouble(), renderHeight.toDouble()))

                exitProcess(status = 0)
            }
        }

    }
}

fun getLineLength(points: List<Circle>): Double =
    points.windowed(2, 2).fold(0.0) { total, (first, second) -> total + first.center.distanceTo(second.center) }

fun exceedsMaxLineLength(linePoints: List<Circle>, maxLineLength: Double?): Boolean =
    if (maxLineLength == null) false else getLineLength(linePoints) >= maxLineLength
