use dioxus_html::{
    geometry::euclid::{Point2D, Size2D},
    MountedData,
};
use wasm_bindgen::JsCast;

use super::{Synthetic, WebEventExt};

impl dioxus_html::RenderedElementBacking for Synthetic<web_sys::Element> {
    fn get_scroll_offset(
        &self,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                Output = dioxus_html::MountedResult<dioxus_html::geometry::PixelsVector2D>,
            >,
        >,
    > {
        let left = self.event.scroll_left();
        let top = self.event.scroll_top();
        let result = Ok(dioxus_html::geometry::PixelsVector2D::new(
            left as f64,
            top as f64,
        ));
        Box::pin(async { result })
    }

    fn get_scroll_size(
        &self,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                Output = dioxus_html::MountedResult<dioxus_html::geometry::PixelsSize>,
            >,
        >,
    > {
        let width = self.event.scroll_width();
        let height = self.event.scroll_height();
        let result = Ok(dioxus_html::geometry::PixelsSize::new(
            width as f64,
            height as f64,
        ));
        Box::pin(async { result })
    }

    fn get_client_rect(
        &self,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                Output = dioxus_html::MountedResult<dioxus_html::geometry::PixelsRect>,
            >,
        >,
    > {
        let rect = self.event.get_bounding_client_rect();
        let result = Ok(dioxus_html::geometry::PixelsRect::new(
            Point2D::new(rect.left(), rect.top()),
            Size2D::new(rect.width(), rect.height()),
        ));
        Box::pin(async { result })
    }

    fn as_any(&self) -> &dyn std::any::Any {
        &self.event
    }

    fn scroll_to(
        &self,
        input_options: dioxus_html::ScrollToOptions,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = dioxus_html::MountedResult<()>>>> {
        let options = web_sys::ScrollIntoViewOptions::new();
        options.set_behavior(match input_options.behavior {
            dioxus_html::ScrollBehavior::Instant => web_sys::ScrollBehavior::Instant,
            dioxus_html::ScrollBehavior::Smooth => web_sys::ScrollBehavior::Smooth,
        });
        options.set_block(match input_options.vertical {
            dioxus_html::ScrollLogicalPosition::Start => web_sys::ScrollLogicalPosition::Start,
            dioxus_html::ScrollLogicalPosition::Center => web_sys::ScrollLogicalPosition::Center,
            dioxus_html::ScrollLogicalPosition::End => web_sys::ScrollLogicalPosition::End,
            dioxus_html::ScrollLogicalPosition::Nearest => web_sys::ScrollLogicalPosition::Nearest,
        });
        options.set_inline(match input_options.horizontal {
            dioxus_html::ScrollLogicalPosition::Start => web_sys::ScrollLogicalPosition::Start,
            dioxus_html::ScrollLogicalPosition::Center => web_sys::ScrollLogicalPosition::Center,
            dioxus_html::ScrollLogicalPosition::End => web_sys::ScrollLogicalPosition::End,
            dioxus_html::ScrollLogicalPosition::Nearest => web_sys::ScrollLogicalPosition::Nearest,
        });
        self.event
            .scroll_into_view_with_scroll_into_view_options(&options);

        Box::pin(async { Ok(()) })
    }

    fn scroll(
        &self,
        coordinates: dioxus_html::geometry::PixelsVector2D,
        behavior: dioxus_html::ScrollBehavior,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = dioxus_html::MountedResult<()>>>> {
        let options = web_sys::ScrollToOptions::new();
        options.set_top(coordinates.y);
        options.set_left(coordinates.x);
        match behavior {
            dioxus_html::ScrollBehavior::Instant => {
                options.set_behavior(web_sys::ScrollBehavior::Instant);
            }
            dioxus_html::ScrollBehavior::Smooth => {
                options.set_behavior(web_sys::ScrollBehavior::Smooth);
            }
        }
        self.event.scroll_with_scroll_to_options(&options);

        Box::pin(async { Ok(()) })
    }

    fn set_focus(
        &self,
        focus: bool,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = dioxus_html::MountedResult<()>>>> {
        #[derive(Debug)]
        struct FocusError(wasm_bindgen::JsValue);

        impl std::fmt::Display for FocusError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "failed to focus element {:?}", self.0)
            }
        }

        impl std::error::Error for FocusError {}

        let result = self
            .event
            .dyn_ref::<web_sys::HtmlElement>()
            .ok_or_else(|| {
                dioxus_html::MountedError::OperationFailed(Box::new(FocusError(
                    self.event.clone().into(),
                )))
            })
            .and_then(|e| {
                (if focus { e.focus() } else { e.blur() }).map_err(|err| {
                    dioxus_html::MountedError::OperationFailed(Box::new(FocusError(err)))
                })
            });
        Box::pin(async { result })
    }
}

impl WebEventExt for MountedData {
    type WebEvent = web_sys::Element;

    #[inline(always)]
    fn try_as_web_event(&self) -> Option<web_sys::Element> {
        self.downcast::<web_sys::Element>().cloned()
    }
}
