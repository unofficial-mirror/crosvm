/* Generated by wayland-scanner 1.18.0 */

#ifndef WP_VIRTIO_GPU_METADATA_V1_CLIENT_PROTOCOL_H
#define WP_VIRTIO_GPU_METADATA_V1_CLIENT_PROTOCOL_H

#include <stdint.h>
#include <stddef.h>
#include "wayland-client.h"

#ifdef  __cplusplus
extern "C" {
#endif

/**
 * @page page_wp_virtio_gpu_metadata_v1 The wp_virtio_gpu_metadata_v1 protocol
 * @section page_ifaces_wp_virtio_gpu_metadata_v1 Interfaces
 * - @subpage page_iface_wp_virtio_gpu_metadata_v1 - attach virtio gpu metadata
 * - @subpage page_iface_wp_virtio_gpu_surface_metadata_v1 - interface to attach virtio gpu metadata to a wl_surface
 * @section page_copyright_wp_virtio_gpu_metadata_v1 Copyright
 * <pre>
 *
 * Copyright 2021 The Chromium Authors.
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 * </pre>
 */
struct wl_surface;
struct wp_virtio_gpu_metadata_v1;
struct wp_virtio_gpu_surface_metadata_v1;

/**
 * @page page_iface_wp_virtio_gpu_metadata_v1 wp_virtio_gpu_metadata_v1
 * @section page_iface_wp_virtio_gpu_metadata_v1_desc Description
 *
 * The global interface which allows attaching virtio-gpu metadata
 * to wl_surface objects.
 * @section page_iface_wp_virtio_gpu_metadata_v1_api API
 * See @ref iface_wp_virtio_gpu_metadata_v1.
 */
/**
 * @defgroup iface_wp_virtio_gpu_metadata_v1 The wp_virtio_gpu_metadata_v1 interface
 *
 * The global interface which allows attaching virtio-gpu metadata
 * to wl_surface objects.
 */
extern const struct wl_interface wp_virtio_gpu_metadata_v1_interface;
/**
 * @page page_iface_wp_virtio_gpu_surface_metadata_v1 wp_virtio_gpu_surface_metadata_v1
 * @section page_iface_wp_virtio_gpu_surface_metadata_v1_desc Description
 *
 * An additional interface to a wl_surface object, which allows the
 * client to attach additional metadata to the surface.
 *
 * If the wl_surface associated with the virtio_gpu_surface_metadata_v1 is
 * destroyed, all virtio_gpu_surface_metadata_v1 requests except 'destroy'
 * raise the protocol error no_surface.
 *
 * If the virtio_gpu_surface_metadata_v1 object is destroyed, the metadata
 * state is removed from the wl_surface. The change will be applied
 * on the next wl_surface.commit.
 * @section page_iface_wp_virtio_gpu_surface_metadata_v1_api API
 * See @ref iface_wp_virtio_gpu_surface_metadata_v1.
 */
/**
 * @defgroup iface_wp_virtio_gpu_surface_metadata_v1 The wp_virtio_gpu_surface_metadata_v1 interface
 *
 * An additional interface to a wl_surface object, which allows the
 * client to attach additional metadata to the surface.
 *
 * If the wl_surface associated with the virtio_gpu_surface_metadata_v1 is
 * destroyed, all virtio_gpu_surface_metadata_v1 requests except 'destroy'
 * raise the protocol error no_surface.
 *
 * If the virtio_gpu_surface_metadata_v1 object is destroyed, the metadata
 * state is removed from the wl_surface. The change will be applied
 * on the next wl_surface.commit.
 */
extern const struct wl_interface wp_virtio_gpu_surface_metadata_v1_interface;

#ifndef WP_VIRTIO_GPU_METADATA_V1_ERROR_ENUM
#define WP_VIRTIO_GPU_METADATA_V1_ERROR_ENUM
enum wp_virtio_gpu_metadata_v1_error {
	/**
	 * the surface already has a metadata object associated
	 */
	WP_VIRTIO_GPU_METADATA_V1_ERROR_SURFACE_METADATA_EXISTS = 0,
};
#endif /* WP_VIRTIO_GPU_METADATA_V1_ERROR_ENUM */

#define WP_VIRTIO_GPU_METADATA_V1_GET_SURFACE_METADATA 0


/**
 * @ingroup iface_wp_virtio_gpu_metadata_v1
 */
#define WP_VIRTIO_GPU_METADATA_V1_GET_SURFACE_METADATA_SINCE_VERSION 1

/** @ingroup iface_wp_virtio_gpu_metadata_v1 */
static inline void
wp_virtio_gpu_metadata_v1_set_user_data(struct wp_virtio_gpu_metadata_v1 *wp_virtio_gpu_metadata_v1, void *user_data)
{
	wl_proxy_set_user_data((struct wl_proxy *) wp_virtio_gpu_metadata_v1, user_data);
}

/** @ingroup iface_wp_virtio_gpu_metadata_v1 */
static inline void *
wp_virtio_gpu_metadata_v1_get_user_data(struct wp_virtio_gpu_metadata_v1 *wp_virtio_gpu_metadata_v1)
{
	return wl_proxy_get_user_data((struct wl_proxy *) wp_virtio_gpu_metadata_v1);
}

static inline uint32_t
wp_virtio_gpu_metadata_v1_get_version(struct wp_virtio_gpu_metadata_v1 *wp_virtio_gpu_metadata_v1)
{
	return wl_proxy_get_version((struct wl_proxy *) wp_virtio_gpu_metadata_v1);
}

/** @ingroup iface_wp_virtio_gpu_metadata_v1 */
static inline void
wp_virtio_gpu_metadata_v1_destroy(struct wp_virtio_gpu_metadata_v1 *wp_virtio_gpu_metadata_v1)
{
	wl_proxy_destroy((struct wl_proxy *) wp_virtio_gpu_metadata_v1);
}

/**
 * @ingroup iface_wp_virtio_gpu_metadata_v1
 *
 * Instantiate an virtio_gpu_surface_metadata_v1 extension for the given
 * wl_surface to attach virtio gpu metadata. If the given wl_surface
 * already has a surface metadata object associated, the
 * surface_metadata_exists protocol error is raised.
 */
static inline struct wp_virtio_gpu_surface_metadata_v1 *
wp_virtio_gpu_metadata_v1_get_surface_metadata(struct wp_virtio_gpu_metadata_v1 *wp_virtio_gpu_metadata_v1, struct wl_surface *surface)
{
	struct wl_proxy *id;

	id = wl_proxy_marshal_constructor((struct wl_proxy *) wp_virtio_gpu_metadata_v1,
			 WP_VIRTIO_GPU_METADATA_V1_GET_SURFACE_METADATA, &wp_virtio_gpu_surface_metadata_v1_interface, NULL, surface);

	return (struct wp_virtio_gpu_surface_metadata_v1 *) id;
}

#ifndef WP_VIRTIO_GPU_SURFACE_METADATA_V1_ERROR_ENUM
#define WP_VIRTIO_GPU_SURFACE_METADATA_V1_ERROR_ENUM
enum wp_virtio_gpu_surface_metadata_v1_error {
	/**
	 * the wl_surface was destroyed
	 */
	WP_VIRTIO_GPU_SURFACE_METADATA_V1_ERROR_NO_SURFACE = 0,
};
#endif /* WP_VIRTIO_GPU_SURFACE_METADATA_V1_ERROR_ENUM */

#define WP_VIRTIO_GPU_SURFACE_METADATA_V1_SET_SCANOUT_ID 0


/**
 * @ingroup iface_wp_virtio_gpu_surface_metadata_v1
 */
#define WP_VIRTIO_GPU_SURFACE_METADATA_V1_SET_SCANOUT_ID_SINCE_VERSION 1

/** @ingroup iface_wp_virtio_gpu_surface_metadata_v1 */
static inline void
wp_virtio_gpu_surface_metadata_v1_set_user_data(struct wp_virtio_gpu_surface_metadata_v1 *wp_virtio_gpu_surface_metadata_v1, void *user_data)
{
	wl_proxy_set_user_data((struct wl_proxy *) wp_virtio_gpu_surface_metadata_v1, user_data);
}

/** @ingroup iface_wp_virtio_gpu_surface_metadata_v1 */
static inline void *
wp_virtio_gpu_surface_metadata_v1_get_user_data(struct wp_virtio_gpu_surface_metadata_v1 *wp_virtio_gpu_surface_metadata_v1)
{
	return wl_proxy_get_user_data((struct wl_proxy *) wp_virtio_gpu_surface_metadata_v1);
}

static inline uint32_t
wp_virtio_gpu_surface_metadata_v1_get_version(struct wp_virtio_gpu_surface_metadata_v1 *wp_virtio_gpu_surface_metadata_v1)
{
	return wl_proxy_get_version((struct wl_proxy *) wp_virtio_gpu_surface_metadata_v1);
}

/** @ingroup iface_wp_virtio_gpu_surface_metadata_v1 */
static inline void
wp_virtio_gpu_surface_metadata_v1_destroy(struct wp_virtio_gpu_surface_metadata_v1 *wp_virtio_gpu_surface_metadata_v1)
{
	wl_proxy_destroy((struct wl_proxy *) wp_virtio_gpu_surface_metadata_v1);
}

/**
 * @ingroup iface_wp_virtio_gpu_surface_metadata_v1
 *
 * Set the virtio gpu scanout id of the associated wl_surface.
 */
static inline void
wp_virtio_gpu_surface_metadata_v1_set_scanout_id(struct wp_virtio_gpu_surface_metadata_v1 *wp_virtio_gpu_surface_metadata_v1, uint32_t scanout_id)
{
	wl_proxy_marshal((struct wl_proxy *) wp_virtio_gpu_surface_metadata_v1,
			 WP_VIRTIO_GPU_SURFACE_METADATA_V1_SET_SCANOUT_ID, scanout_id);
}

#ifdef  __cplusplus
}
#endif

#endif