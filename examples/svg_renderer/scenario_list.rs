use super::quicksketch::{
    ArrowType, HorizontalAlignment, Point, Sketch, SketchColor, SketchElement, SketchFill,
    StrokeStyle, Vector,
};
use cgmath::{Angle, Bounded, Deg};
use spade::{
    handles::{
        FixedDirectedEdgeHandle,
        VoronoiVertex::{self, Inner, Outer},
    },
    ConstrainedDelaunayTriangulation, InsertionError, Triangulation as _,
};

use crate::{
    convert_point,
    scenario::{
        convert_triangulation, ConversionOptions, DirectedEdgeMode, DirectedEdgeType, FaceType,
        Triangulation, UndirectedEdgeType, VertexType,
    },
};

fn big_triangulation() -> Result<Triangulation, InsertionError> {
    let mut result = Triangulation::new();

    result.insert(VertexType::new(15.0, 39.0))?;
    result.insert(VertexType::new(-5.0, -42.0))?;
    result.insert(VertexType::new(30.0, 2.0))?;
    result.insert(VertexType::new(5.0, -12.0))?;
    result.insert(VertexType::new(-35.0, 32.0))?;
    result.insert(VertexType::new(-2.0, -24.0))?;
    result.insert(VertexType::new(-67.0, 52.0))?;
    result.insert(VertexType::new(-14.0, -52.0))?;
    result.insert(VertexType::new(76.0, 10.0))?;
    result.insert(VertexType::new(12.0, 10.0))?;
    result.insert(VertexType::new(-35.0, 5.0))?;
    result.insert(VertexType::new(33.0, -30.0))?;
    result.insert(VertexType::new(-30.0, -25.0))?;
    result.insert(VertexType::new(-55.0, -25.0))?;
    result.insert(VertexType::new(50.0, 50.0))?;
    result.insert(VertexType::new(45.0, -47.0))?;

    Ok(result)
}

fn add_circular_arrows(sketch: &mut Sketch, center: Point, is_lhs: bool, radius: f64) {
    let angle_to_position =
        |angle: Deg<f64>| center + Vector::new(angle.sin(), -angle.cos()) * radius;

    const WIDTH: f64 = 1.0;
    const COLOR: SketchColor = SketchColor::ROYAL_BLUE;
    const ARROW_TYPE: ArrowType = ArrowType::FilledArrow;

    let zero = Deg(0.0);

    let radii = Vector::new(radius, radius);
    let arrow = SketchElement::path()
        .stroke_width(WIDTH)
        .stroke_color(COLOR);

    let arrow1 = arrow.clone().move_to(angle_to_position(zero)).arc_to(
        radii,
        zero,
        false,
        true,
        angle_to_position(Deg(140.0)),
    );
    let arrow1 = if is_lhs {
        arrow1.with_arrow_end(ARROW_TYPE)
    } else {
        arrow1.with_arrow_start(ARROW_TYPE)
    };

    let arrow2 = arrow.move_to(angle_to_position(Deg(180.0))).arc_to(
        radii,
        zero,
        false,
        true,
        angle_to_position(Deg(320.0)),
    );

    let arrow2 = if is_lhs {
        arrow2.with_arrow_end(ARROW_TYPE)
    } else {
        arrow2.with_arrow_start(ARROW_TYPE)
    };

    sketch.add(arrow1);
    sketch.add(arrow2);
}

pub fn lhs_rhs_scenario(is_lhs: bool) -> Sketch {
    let mut triangulation = Triangulation::new();

    triangulation.insert(VertexType::new(-67.0, 52.0)).unwrap();
    triangulation.insert(VertexType::new(-40.0, -24.0)).unwrap();
    triangulation.insert(VertexType::new(-30.0, 20.0)).unwrap();
    triangulation.insert(VertexType::new(-6.0, -42.0)).unwrap();
    triangulation.insert(VertexType::new(-5.0, 0.0)).unwrap();
    triangulation.insert(VertexType::new(15.0, 42.0)).unwrap();
    triangulation.insert(VertexType::new(30.0, 2.0)).unwrap();

    let mut sketch = convert_triangulation(
        &triangulation,
        &ConversionOptions {
            directed_edge_mode: DirectedEdgeMode::Enabled { reversed: is_lhs },
            ..Default::default()
        },
    );

    for face in triangulation.inner_faces() {
        let circumcenter = face.center();
        add_circular_arrows(&mut sketch, convert_point(circumcenter), is_lhs, 4.5);
    }

    sketch.set_width(420);

    sketch
}

pub fn circumcircle_scenario() -> Sketch {
    let triangulation = big_triangulation().unwrap();

    let mut sketch = convert_triangulation(&triangulation, &Default::default());

    for face in triangulation.inner_faces().skip(5).take(8) {
        let (circumcenter, radius_squared) = face.circumcircle();
        if radius_squared < 880.0 {
            sketch.add(
                SketchElement::circle(convert_point(circumcenter), radius_squared.sqrt())
                    .stroke_width(0.5)
                    .stroke_color(SketchColor::ROYAL_BLUE),
            );
        }
    }

    sketch.set_width(500);
    sketch
}

pub fn outer_face_scenario() -> Sketch {
    let mut triangulation = Triangulation::default();

    triangulation.insert(VertexType::new(50.0, 10.0)).unwrap();
    triangulation.insert(VertexType::new(40.0, -35.0)).unwrap();
    triangulation.insert(VertexType::new(-50.0, 50.0)).unwrap();
    triangulation.insert(VertexType::new(-20.0, 0.0)).unwrap();
    triangulation.insert(VertexType::new(35.0, 45.0)).unwrap();

    let mut sketch = convert_triangulation(&triangulation, &Default::default());

    for face in triangulation.inner_faces() {
        let center = face.center();
        const FONT_SIZE: f64 = 10.0;

        sketch.add(
            SketchElement::text("inner")
                .position(convert_point(center))
                .horizontal_alignment(HorizontalAlignment::Middle)
                .font_size(FONT_SIZE),
        );

        sketch.add(
            SketchElement::text("outer")
                .position(Point::new(-40.0, -25.0))
                .font_size(FONT_SIZE),
        );
    }

    sketch.set_width(400);
    sketch.set_relative_padding(0.05);

    sketch
}

pub fn basic_voronoi_example() -> Sketch {
    let triangulation = big_triangulation().unwrap();

    let mut sketch = convert_triangulation(&triangulation, &Default::default());
    const LINE_COLOR: SketchColor = SketchColor::ROYAL_BLUE;

    for edge in triangulation.undirected_voronoi_edges() {
        match edge.vertices() {
            [Inner(from), Inner(to)] => {
                sketch.add(
                    SketchElement::line(
                        convert_point(from.circumcenter()),
                        convert_point(to.circumcenter()),
                    )
                    .stroke_color(LINE_COLOR),
                );
            }
            [Inner(from), Outer(edge)] | [Outer(edge), Inner(from)] => {
                let from = convert_point(from.circumcenter());
                let to_direction = edge.direction_vector();
                let to_direction = Vector::new(to_direction.x, to_direction.y);

                sketch.add(
                    SketchElement::line(from, from + to_direction * 4.0)
                        .stroke_color(LINE_COLOR)
                        .stroke_style(StrokeStyle::Dashed),
                );
            }
            [Outer(_), Outer(_)] => {}
        }
    }

    let mut min: Point = Bounded::max_value();
    let mut max: Point = Bounded::min_value();

    for vertex in triangulation.vertices() {
        let position = vertex.position();
        min = min.zip(convert_point(position), f64::min);
        max = max.zip(convert_point(position), f64::max);
    }

    min.y -= 5.0;
    max.y -= 15.0;

    sketch
        .set_width(700)
        .set_relative_padding(0.24)
        .set_view_box_min(min)
        .set_view_box_max(max);
    sketch
}

pub fn voronoi_edge_details_scenario() -> Sketch {
    let mut triangulation = Triangulation::new();

    triangulation.insert(VertexType::new(-50.0, -50.0)).unwrap();
    triangulation.insert(VertexType::new(-41.0, -60.0)).unwrap();
    triangulation.insert(VertexType::new(-41.0, 61.0)).unwrap();
    triangulation.insert(VertexType::new(-40.0, -40.0)).unwrap();
    triangulation.insert(VertexType::new(-30.0, 25.0)).unwrap();
    let vertex = triangulation.insert(VertexType::new(30.0, 0.0)).unwrap();
    triangulation.insert(VertexType::new(40.0, 20.0)).unwrap();
    triangulation.insert(VertexType::new(41.0, -70.0)).unwrap();
    triangulation.insert(VertexType::new(41.0, -80.0)).unwrap();
    triangulation.insert(VertexType::new(41.0, -20.0)).unwrap();
    triangulation.insert(VertexType::new(107.0, -20.0)).unwrap();
    triangulation.insert(VertexType::new(90.0, -40.0)).unwrap();
    triangulation.insert(VertexType::new(90.0, 60.0)).unwrap();

    for undirected_edge in triangulation.fixed_undirected_edges() {
        triangulation
            .undirected_edge_data_mut(undirected_edge)
            .color = SketchColor::DARK_GRAY;
    }

    let mut sketch = convert_triangulation(
        &triangulation,
        &ConversionOptions {
            vertex_stroke_color: SketchColor::DARK_GRAY,
            vertex_color: SketchColor::GRAY,
            ..Default::default()
        },
    );

    const LINE_COLOR: SketchColor = SketchColor::ROYAL_BLUE;

    for face in triangulation.inner_faces() {
        let center = convert_point(face.circumcenter());
        sketch.add(
            SketchElement::circle(center, 2.0)
                .fill(SketchFill::solid(SketchColor::ROYAL_BLUE))
                .stroke_color(SketchColor::BLACK)
                .stroke_width(0.5),
        );
    }

    let example_face = triangulation.vertex(vertex).as_voronoi_face();

    const SHIFT: f64 = -2.5;

    let create_line =
        |from: VoronoiVertex<VertexType, DirectedEdgeType, UndirectedEdgeType, FaceType>,
         to: VoronoiVertex<VertexType, DirectedEdgeType, UndirectedEdgeType, FaceType>| {
            let from = from.as_delaunay_face().unwrap().circumcenter();
            let to = to.as_delaunay_face().unwrap().circumcenter();

            SketchElement::line(convert_point(from), convert_point(to))
                .stroke_color(LINE_COLOR)
                .draw_double_line()
                .with_arrow_start(ArrowType::HalfArrow)
                .shift_from_and_to(SHIFT)
        };

    for edge in triangulation.undirected_voronoi_edges() {
        if let [Inner(from), Inner(to)] = edge.vertices() {
            let directed = edge.as_directed();
            if directed.face() == example_face || directed.rev().face() == example_face {
                // Edges of the example face are drawn manually
                continue;
            }

            sketch.add(create_line(Inner(from), Inner(to)));
        }
    }

    let edge = example_face.adjacent_edges().next().unwrap().next().next();

    let main_edge = create_line(edge.from(), edge.to());

    // main edge and rev
    sketch.add(
        main_edge
            .create_adjacent_text("edge")
            .font_size(5.0)
            .dy(-3.3),
    );

    let rev_text = create_line(edge.to(), edge.from())
        .create_adjacent_text("edge.rev()")
        .font_size(5.0)
        .dy(5.7);
    sketch.add(rev_text);
    sketch.add(main_edge);

    // next
    let next = edge.prev(); // Take prev instead of next since SVG renders everything with a LHS
    let next_edge = create_line(next.from(), next.to());

    sketch.add(
        next_edge
            .create_adjacent_text("edge.next()")
            .font_size(5.0)
            .dy(5.3),
    );
    sketch.add(next_edge);

    // auxiliary edge
    let aux = next.prev();
    sketch.add(create_line(aux.from(), aux.to()));

    // prev
    let prev = aux.prev();
    let prev_line = create_line(prev.from(), prev.to());
    sketch.add(
        prev_line
            .create_adjacent_text("edge.prev()")
            .font_size(5.0)
            .dy(-3.2),
    );
    sketch.add(prev_line);

    sketch.set_view_box_min(Point::new(-11.0, -30.0));
    sketch.set_view_box_max(Point::new(56.0, 27.0));
    sketch.set_width(400);

    sketch
}

fn delaunay_edge_details_triangulation(
) -> Result<(Triangulation, FixedDirectedEdgeHandle), InsertionError> {
    let mut triangulation = Triangulation::new();

    triangulation.insert(VertexType::new(-50.0, -50.0))?;
    triangulation.insert(VertexType::new(-41.0, -60.0))?;
    triangulation.insert(VertexType::new(-41.0, 61.0))?;
    triangulation.insert(VertexType::new(-40.0, -40.0))?;
    triangulation.insert(VertexType::new(-30.0, 25.0))?;
    let to = triangulation.insert(VertexType::new(0.0, 0.0))?;
    let from = triangulation.insert(VertexType::new(40.0, 20.0))?;
    triangulation.insert(VertexType::new(41.0, -70.0))?;
    triangulation.insert(VertexType::new(41.0, -80.0))?;
    triangulation.insert(VertexType::new(41.0, -20.0))?;
    triangulation.insert(VertexType::new(107.0, -20.0))?;
    triangulation.insert(VertexType::new(90.0, -40.0))?;
    triangulation.insert(VertexType::new(90.0, 60.0))?;

    let edge = triangulation
        .get_edge_from_neighbors(from, to)
        .unwrap()
        .fix();
    Ok((triangulation, edge))
}

pub fn delaunay_directed_edge_details_scenario() -> Sketch {
    let (triangulation, edge) = delaunay_edge_details_triangulation().unwrap();
    let edge = triangulation.directed_edge(edge);

    let mut sketch = convert_triangulation(
        &triangulation,
        &ConversionOptions {
            directed_edge_mode: DirectedEdgeMode::Enabled { reversed: false },
            ..Default::default()
        },
    );

    let from_pos = convert_point(edge.from().position());
    let to_pos = convert_point(edge.to().position());

    const FONT_SIZE: f64 = 5.0;
    const DY: f64 = -2.1;
    let edge_label = SketchElement::line(from_pos, to_pos)
        .create_adjacent_text("e")
        .font_size(FONT_SIZE)
        .dy(DY);

    sketch.add(edge_label);
    let rev_label = SketchElement::line(from_pos, to_pos)
        .create_adjacent_text("e.rev()")
        .font_size(FONT_SIZE)
        .dy(5.0);

    sketch.add(rev_label);

    let next = edge.prev(); // Use prev since SVG uses a left handed coordinate system
    let next_label = SketchElement::line(from_pos, convert_point(next.from().position()))
        .create_adjacent_text("e.next()")
        .font_size(FONT_SIZE)
        .dy(DY);

    sketch.add(next_label);

    let prev = edge.next(); // Use prev since SVG uses a left handed coordinate system
    let prev_label = SketchElement::line(convert_point(prev.to().position()), to_pos)
        .create_adjacent_text("e.prev()")
        .font_size(FONT_SIZE)
        .dy(5.2);

    sketch.add(prev_label);

    sketch.set_view_box_min(Point::new(-5.0, -20.0));
    sketch.set_view_box_max(Point::new(46.0, 17.0));
    sketch.set_width(300);

    sketch
}

pub fn delaunay_directed_edge_vertex_and_face_scenario() -> Sketch {
    let (mut triangulation, edge) = delaunay_edge_details_triangulation().unwrap();

    let face_fixed = triangulation.directed_edge(edge).face().fix();
    triangulation.face_data_mut(face_fixed).fill =
        SketchFill::StripePattern(SketchColor::LIGHT_GRAY, SketchColor::DARK_GRAY, 5.0);

    let mut sketch = convert_triangulation(
        &triangulation,
        &ConversionOptions {
            directed_edge_mode: DirectedEdgeMode::Enabled { reversed: false },
            ..Default::default()
        },
    );

    let edge = triangulation.directed_edge(edge);
    let face = edge.face().as_inner().unwrap();

    let from_pos = convert_point(edge.from().position());
    let to_pos = convert_point(edge.to().position());

    const FONT_SIZE: f64 = 5.0;
    const DY: f64 = -2.1;
    let edge_label = SketchElement::line(from_pos, to_pos)
        .create_adjacent_text("e")
        .font_size(FONT_SIZE)
        .dy(DY);

    sketch.add(edge_label);

    let face_position = convert_point(face.circumcenter());

    sketch.add(
        SketchElement::text("e.face()")
            .position(face_position)
            .horizontal_alignment(HorizontalAlignment::Middle)
            .font_size(FONT_SIZE),
    );

    let edge_from_position = convert_point(edge.to().position());
    sketch.add(
        SketchElement::text("e.from()")
            .position(edge_from_position + Vector::new(-7.0, 16.0))
            .font_size(FONT_SIZE),
    );

    sketch.add(
        SketchElement::path()
            .move_to(edge_from_position + Vector::new(1.8, 6.0))
            .line_to(edge_from_position + Vector::new(4.0, 12.0))
            .stroke_color(SketchColor::BLACK)
            .with_arrow_start(ArrowType::FilledArrow)
            .stroke_style(StrokeStyle::SmallDashed)
            .stroke_width(0.7),
    );

    let edge_to_position = convert_point(edge.from().position());
    sketch.add(
        SketchElement::text("e.to()")
            .position(edge_to_position + Vector::new(-7.0, 16.0))
            .font_size(FONT_SIZE),
    );

    sketch.add(
        SketchElement::path()
            .move_to(edge_to_position + Vector::new(0.5, 6.0))
            .line_to(edge_to_position + Vector::new(0.5, 11.0))
            .stroke_color(SketchColor::BLACK)
            .with_arrow_start(ArrowType::FilledArrow)
            .stroke_style(StrokeStyle::SmallDashed)
            .stroke_width(0.7),
    );

    sketch.set_view_box_min(Point::new(-5.0, -20.0));
    sketch.set_view_box_max(Point::new(46.0, 35.0));
    sketch.set_width(300);

    sketch
}

pub fn cdt_scenario() -> Sketch {
    let create_cdt = |offset| {
        let mut cdt = ConstrainedDelaunayTriangulation::<
            VertexType,
            DirectedEdgeType,
            UndirectedEdgeType,
            FaceType,
        >::new();
        let v = |x, y| VertexType::new(x + offset, y);

        cdt.insert(v(-50.0, -50.0)).unwrap();
        cdt.insert(v(-41.0, -60.0)).unwrap();
        cdt.insert(v(-41.0, 61.0)).unwrap();
        cdt.insert(v(-20.0, -40.0)).unwrap();
        cdt.insert(v(-10.0, 15.0)).unwrap();
        cdt.insert(v(0.0, 0.0)).unwrap();
        cdt.insert(v(40.0, 20.0)).unwrap();
        cdt.insert(v(41.0, -60.0)).unwrap();
        cdt.insert(v(41.0, -70.0)).unwrap();
        cdt.insert(v(41.0, -20.0)).unwrap();
        cdt.insert(v(65.0, -20.0)).unwrap();
        cdt.insert(v(70.0, -40.0)).unwrap();
        cdt.insert(v(70.0, 60.0)).unwrap();
        cdt.insert(v(-20.0, 40.0)).unwrap();
        cdt.insert(v(35.0, 35.0)).unwrap();
        cdt.insert(v(35.0, -35.0)).unwrap();
        cdt.insert(v(-35.0, -35.0)).unwrap();
        cdt.insert(v(-35.0, 35.0)).unwrap();
        cdt
    };

    let mut cdt = create_cdt(0.0);
    let v0 = cdt.insert(VertexType::new(35.0, 35.0)).unwrap();
    let v1 = cdt.insert(VertexType::new(35.0, -35.0)).unwrap();
    let v2 = cdt.insert(VertexType::new(-35.0, -35.0)).unwrap();
    let v3 = cdt.insert(VertexType::new(-35.0, 35.0)).unwrap();

    cdt.add_constraint(v0, v1);
    cdt.add_constraint(v1, v2);
    cdt.add_constraint(v2, v3);
    cdt.add_constraint(v3, v0);

    for edge in cdt.fixed_undirected_edges() {
        if cdt.is_constraint_edge(edge) {
            cdt.undirected_edge_data_mut(edge).data_mut().color = SketchColor::DARK_RED;
        }
    }

    let mut sketch = convert_triangulation(&cdt, &Default::default());

    let cdt2 = create_cdt(140.0);
    let sketch2 = convert_triangulation(&cdt2, &Default::default());

    sketch.items.extend(sketch2.items);

    sketch.add(
        SketchElement::text("CDT")
            .position(Point::new(-30.0, -67.0))
            .horizontal_alignment(HorizontalAlignment::Middle)
            .font_size(13.0),
    );

    sketch.add(
        SketchElement::text("No CDT")
            .position(Point::new(120.0, -67.0))
            .horizontal_alignment(HorizontalAlignment::Middle)
            .font_size(13.0),
    );

    sketch.set_width(935);
    sketch.set_relative_padding(0.03);
    sketch
}

pub fn circular_iterator_example() -> Sketch {
    let mut triangulation = Triangulation::new();

    let mut v0 = VertexType::new(0.0, 0.0);
    v0.radius = 3.5;
    let v0 = triangulation.insert(v0).unwrap();
    triangulation.insert(VertexType::new(66.0, -5.0)).unwrap();
    triangulation.insert(VertexType::new(6.0, 40.0)).unwrap();
    triangulation.insert(VertexType::new(-55.0, 5.0)).unwrap();
    triangulation.insert(VertexType::new(60.0, 40.0)).unwrap();
    triangulation.insert(VertexType::new(-45.0, 25.0)).unwrap();
    triangulation.insert(VertexType::new(45.0, -40.0)).unwrap();
    triangulation.insert(VertexType::new(-49.0, -30.0)).unwrap();

    let mut sketch = convert_triangulation(
        &triangulation,
        &ConversionOptions {
            directed_edge_mode: DirectedEdgeMode::Enabled { reversed: false },
            ..Default::default()
        },
    );

    for (index, edge) in triangulation.vertex(v0).out_edges().enumerate() {
        let from = convert_point(edge.from().position());
        let to = convert_point(edge.to().position());
        let dy = match index {
            0 | 2 | 1 | 6 => -2.0,
            _ => 7.0,
        };

        sketch.add(
            SketchElement::line(from, to)
                .create_adjacent_text(format!("e{}", 6 - index))
                .dy(dy)
                .horizontal_alignment(HorizontalAlignment::Middle)
                .font_size(8.0),
        );
    }

    sketch.add(
        SketchElement::text("v")
            .horizontal_alignment(HorizontalAlignment::Middle)
            .dy(2.1)
            .font_size(7.0),
    );

    add_circular_arrows(&mut sketch, Point::new(0.0, 0.0), false, 10.0);

    sketch.set_relative_padding(-0.01);
    sketch.set_width(400);

    sketch
}