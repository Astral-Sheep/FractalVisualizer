#version 450

#pragma optionNV(fastmatch off)
#pragma optionNV(fastprecision off)

layout(set=0, binding=0) uniform Center { vec2 center; };
layout(set=0, binding=1) uniform Offset
{
	vec2 offset_x;
	vec2 offset_y;
};
layout(set=0, binding=2) uniform Zoom { float zoom; };
layout(location=0) out vec4 f_color;

//int mod(int value, int mod)
//{
//	return value - int(floor(value / mod)) * mod;
//}

//vec4[7] get_pallet_7(int index)
//{
//    vec4 pallets[1][7] = {
//        // RGB
//        vec4[7] (
//            vec4(1.0, 0.0, 0.0, 1.0),
//            vec4(1.0, 1.0, 0.0, 1.0),
//            vec4(0.0, 1.0, 0.0, 1.0),
//            vec4(0.0, 1.0, 1.0, 1.0),
//            vec4(0.0, 0.0, 1.0, 1.0),
//            vec4(1.0, 0.0, 1.0, 1.0),
//            vec4(1.0, 0.0, 0.0, 1.0)
//        )
//    };
//
//    return pallets[index];
//}

//vec4[6] get_pallet_6(int index)
//{
//	vec4 pallets[1][6] = {
//		// Original (blue & yellow)
//		vec4[6] (
//			vec4(0.0, 7.0 / 255.0, 100.0 / 255.0, 1.0),
//			vec4(32.0 / 255.0, 107.0 / 255.0, 203.0 / 255.0, 1.0),
//			vec4(237.0 / 255.0, 1.0 , 1.0, 1.0),
//			vec4(1.0, 170.0 / 255.0, 0.0, 1.0),
//			vec4(0.0, 2.0 / 255.0, 0.0, 1.0),
//			vec4(0.0, 7.0 / 255.0, 100.0 / 255.0, 1.0)
//		)
//	};
//
//	return pallets[index];
//}

//vec4[5] get_pallet_5(int index)
//{
//    vec4 pallets[3][5] = {
//        // Fire
//        vec4[5] (
//            vec4(20.0 / 255.0, 0.0, 0.0, 1.0),
//            vec4(1.0, 20.0 / 255.0, 0.0, 1.0),
//            vec4(1.0, 200.0 / 255.0, 0.0, 1.0),
//            vec4(1.0, 20.0 / 255.0, 0.0, 1.0),
//            vec4(20.0 / 255.0, 0.0, 0.0, 1.0)
//        ),
//        // Electric
//        vec4[5] (
//            vec4(0.0, 0.0, 0.0, 1.0),
//            vec4(0.0, 0.0, 200.0 / 255.0, 1.0),
//            vec4(1.0, 1.0, 1.0, 1.0),
//            vec4(0.0, 0.0, 200.0 / 255.0, 1.0),
//            vec4(0.0, 0.0, 0.0, 1.0)
//        ),
//        // Gold
//        vec4[5] (
//            vec4(85.0 / 255.0, 47.0 / 255.0, 0.0, 1.0),
//            vec4(255.0 / 255.0, 171.0 / 255.0, 12.0, 1.0),
//            vec4(255.0 / 255.0, 247.0 / 255.0, 127.0, 1.0),
//            vec4(255.0 / 255.0, 171.0 / 255.0, 12.0, 1.0),
//            vec4(85.0 / 255.0, 47.0 / 255.0, 0.0, 1.0)
//        )
//    };
//
//    return pallets[index];
//}

//vec4[3] get_pallet_3(int index)
//{
//    vec4 pallets[1][3] = {
//        vec4[3] (
//            vec4(0.0, 0.0, 0.0, 1.0),
//            vec4(1.0, 1.0, 1.0, 1.0),
//            vec4(0.0, 0.0, 0.0, 1.0)
//        )
//    };
//
//    return pallets[index];
//}

//vec4 get_color_7(float ratio, int pallet_index)
//{
//    int color_nb = 6;
//    vec4 pallet[7] = get_pallet_7(pallet_index);
//
//	vec4 color = vec4(1.0, 1.0, 1.0, 1.0);
//	float min_value;
//	float max_value;
//
//	for (int i = 0; i < color_nb; i++)
//	{
//		min_value = float(i) / color_nb;
//		max_value = float(i + 1) / color_nb;
//
//		if (ratio >= min_value && ratio <= max_value)
//		{
//			color = mix(pallet[i], pallet[mod(i + 1, color_nb)], (ratio - min_value) * color_nb);
//			break;
//		}
//	}
//
//	return color;
//}

//vec4 get_color_6(float ratio, int pallet_index)
//{
//	int color_nb = 6;
//	vec4 pallet[6] = get_pallet_6(pallet_index);
//
//	vec4 color = vec4(1.0, 1.0, 1.0, 1.0);
//	float min_value;
//	float max_value;
//
//	for (int i = 0; i < color_nb; i++)
//	{
//		min_value = float(i) / color_nb;
//		max_value = float(i + 1) / color_nb;
//
//		if (ratio >= min_value && ratio <= max_value)
//		{
//			color = mix(pallet[i], pallet[mod(i + 1, color_nb)], (ratio - min_value) * color_nb);
//			break;
//		}
//	}
//
//	return color;
//}

//vec4 get_color_5(float ratio, int pallet_index)
//{
//    int color_nb = 6;
//    vec4 pallet[5] = get_pallet_5(pallet_index);
//
//	vec4 color = vec4(1.0, 1.0, 1.0, 1.0);
//	float min_value;
//	float max_value;
//
//	for (int i = 0; i < color_nb; i++)
//	{
//		min_value = float(i) / color_nb;
//		max_value = float(i + 1) / color_nb;
//
//		if (ratio >= min_value && ratio <= max_value)
//		{
//			color = mix(pallet[i], pallet[mod(i + 1, color_nb)], (ratio - min_value) * color_nb);
//			break;
//		}
//	}
//
//	return color;
//}

//vec4 get_color_3(float ratio, int pallet_index)
//{
//    int color_nb = 6;
//    vec4 pallet[3] = get_pallet_3(pallet_index);
//
//	vec4 color = vec4(1.0, 1.0, 1.0, 1.0);
//	float min_value;
//	float max_value;
//
//	for (int i = 0; i < color_nb; i++)
//	{
//		min_value = float(i) / color_nb;
//		max_value = float(i + 1) / color_nb;
//
//		if (ratio >= min_value && ratio <= max_value)
//		{
//			color = mix(pallet[i], pallet[mod(i + 1, color_nb)], (ratio - min_value) * color_nb);
//			break;
//		}
//	}
//
//	return color;
//}

// Double precision emulator

struct f64
{
	float high;
	float low;
};

struct vec2f64
{
	f64 x;
	f64 y;
};

f64 floatToDouble(float v)
{
	return f64(v, 0.0);
}

f64 add(f64 lhs, f64 rhs)
{
	f64 result;
	float t1, t2, e;

	t1 = lhs.high + rhs.high;
	e = t1 - lhs.high;
	t2 = ((rhs.high - e) + (lhs.high - (t1 - e))) + lhs.low + rhs.low;

	result.high = t1 + t2;
	result.low = t2 - (result.high - t1);

	return result;
}

f64 sub(f64 lhs, f64 rhs)
{
	f64 negRhs = f64(-rhs.high, -rhs.low);
	return add(lhs, negRhs);
}

f64 mul(f64 lhs, f64 rhs)
{
	f64 result;
	float c11, c21, c2, e, t1, t2;
	float a1, a2, b1, b2, cona, conb, split = 8193.0;

	cona = lhs.high * split;
	conb = rhs.high * split;
	a1 = cona - (cona - lhs.high);
	b1 = conb - (conb - rhs.high);
	a2 = lhs.high - a1;
	b2 = rhs.high - b1;

	c11 = lhs.high * rhs.high;
	c21 = a2 * b2 + (a2 * b1 + (a1 * b2 + (a1 * b1 - c11)));

	c2 = lhs.high * rhs.low + lhs.low * rhs.high;

	t1 = c11 + c2;
	e = t1 - c11;
	t2 = lhs.low * rhs.low + ((c2 - e) + (c11 - (t1 - e))) + c21;

	result.high = t1 + t2;
	result.low = t2 - (result.high - t1);

	return result;
}

float length64(vec2f64 v)
{
	return sqrt((v.x.high * v.x.high + 2.0 * v.x.high + v.x.low + v.x.low * v.x.low) + (v.y.high * v.y.high + 2.0 * v.y.high * v.y.low + v.y.low * v.y.low));
}

// End double precision emulator


void main()
{
	const float initZoom = 1000.0;
	float scaledZoom = zoom * initZoom;

	vec2f64 dgl_FragCoord = vec2f64(
		floatToDouble(gl_FragCoord.x / scaledZoom),
		floatToDouble(gl_FragCoord.y / scaledZoom)
	);
	vec2f64 dcenter = vec2f64(
		floatToDouble(center.x / scaledZoom),
		floatToDouble(center.y / scaledZoom)
	);
	vec2f64 doffset = vec2f64(
		f64(offset_x.x / initZoom, offset_x.y / initZoom),
		f64(offset_y.x / initZoom, offset_y.y / initZoom)
	);

	vec2f64 norm_coordinates = vec2f64(
		sub(sub(dgl_FragCoord.x, dcenter.x), doffset.x),
		sub(sub(dgl_FragCoord.y, dcenter.y), doffset.y)
	);

	vec2f64 z = vec2f64(floatToDouble(0.0), floatToDouble(0.0));
//	vec2f64 z = norm_coordinates;

	vec2f64 c = vec2f64(
		sub(mul(norm_coordinates.x, floatToDouble(2.0)), floatToDouble(1.0)),
		sub(mul(norm_coordinates.y, floatToDouble(2.0)), floatToDouble(0.0))
	);
//	vec2f64 c = vec2f64(floatToDouble(-0.8), floatToDouble(0.156));

	float i;

	for (i = 0.0; i < 1.0; i += 0.005)
	{
		z = vec2f64(
			add(sub(mul(z.x, z.x), mul(z.y, z.y)), c.x),
			add(mul(mul(z.x, z.y), floatToDouble(2.0)), c.y)
		);

		if (length64(z) > 4.0)
		{
			break;
		}
	}

	f_color = vec4(vec3(i), 1.0);
}