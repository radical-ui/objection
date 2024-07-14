import { Component, React } from '../runtime/mod.ts'
import { ActionBlockRender } from './action_blocker/mod.tsx'
import { ActionScopeRender } from './action_scope/mod.tsx'
import { BreadcrumbsRender } from './breadcrumbs/mod.tsx'
import { ButtonRender } from './button/mod.tsx'
import { CardRender } from './card/mod.tsx'
import { CenterRender } from './center/mod.tsx'
import { CenterLayoutRender } from './center_layout/mod.tsx'
import { CheckboxInputRender } from './checkbox_input/mod.tsx'
import { CircleProgressRender } from './circle_progress/mod.tsx'
import { DividerRender } from './divider/mod.tsx'
import { FlexRender } from './flex/mod.tsx'
import { FragmentRender } from './fragment/mod.tsx'
import { SimpleLayoutRender } from './header/mod.tsx'
import { IconRender } from './icon/mod.tsx'
import { IconButtonRender } from './icon_button/mod.tsx'
import { ImageRender } from './image/mod.tsx'
import { LabelRender } from './label/mod.tsx'
import { ModalRender } from './modal/mod.tsx'
import { NestedFlowRender } from './nested_flow/mod.tsx'
import { PaddingRender } from './padding/mod.tsx'
import { PreviewBoxRender } from './preview_box/mod.tsx'
import { RadioInputRender } from './radio_input/mod.tsx'
import { ScrollableBoxRender } from './scrollable_box/mod.tsx'
import { SidebarLayoutRender } from './sidebar_layout/mod.tsx'
import { SkeletonRender } from './skeleton/mod.tsx'
import { TableRender } from './table/mod.tsx'
import { TextInputRender } from './text_input/mod.tsx'
import { UpdateBoundaryRender } from './update_boundary/mod.tsx'

export function ComponentRender(component: Component) {
	if (component.type === 'ActionBlocker') return <ActionBlockRender {...component.def} />
	if (component.type === 'ActionScope') return <ActionScopeRender {...component.def} />
	if (component.type === 'Breadcrumbs') return <BreadcrumbsRender {...component.def} />
	if (component.type === 'Button') return <ButtonRender {...component.def} />
	if (component.type === 'Card') return <CardRender {...component.def} />
	if (component.type === 'Center') return <CenterRender {...component.def} />
	if (component.type === 'CenterLayout') return <CenterLayoutRender {...component.def} />
	if (component.type === 'CheckboxInput') return <CheckboxInputRender {...component.def} />
	if (component.type === 'CircleProgress') return <CircleProgressRender {...component.def} />
	if (component.type === 'Divider') return <DividerRender {...component.def} />
	if (component.type === 'Flex') return <FlexRender {...component.def} />
	if (component.type === 'Fragment') return <FragmentRender />
	if (component.type === 'Header') return <SimpleLayoutRender {...component.def} />
	if (component.type === 'Icon') return <IconRender {...component.def} />
	if (component.type === 'IconButton') return <IconButtonRender {...component.def} />
	if (component.type === 'Image') return <ImageRender {...component.def} />
	if (component.type === 'Label') return <LabelRender {...component.def} />
	if (component.type === 'Modal') return <ModalRender {...component.def} />
	if (component.type === 'NestedFlow') return <NestedFlowRender {...component.def} />
	if (component.type === 'Padding') return <PaddingRender {...component.def} />
	if (component.type === 'PreviewBox') return <PreviewBoxRender {...component.def} />
	if (component.type === 'RadioInput') return <RadioInputRender {...component.def} />
	if (component.type === 'ScrollableBox') return <ScrollableBoxRender {...component.def} />
	if (component.type === 'SidebarLayout') return <SidebarLayoutRender {...component.def} />
	if (component.type === 'Skeleton') return <SkeletonRender {...component.def} />
	if (component.type === 'Table') return <TableRender {...component.def} />
	if (component.type === 'TextInput') return <TextInputRender {...component.def} />
	if (component.type === 'UpdateBoundary') return <UpdateBoundaryRender {...component.def} />

	return <>Unknown Component</>
}

export * from './action_blocker/mod.tsx'
export * from './action_scope/mod.tsx'
export * from './breadcrumbs/mod.tsx'
export * from './button/mod.tsx'
export * from './card/mod.tsx'
export * from './center/mod.tsx'
export * from './center_layout/mod.tsx'
export * from './checkbox_input/mod.tsx'
export * from './circle_progress/mod.tsx'
export * from './divider/mod.tsx'
export * from './flex/mod.tsx'
export * from './fragment/mod.tsx'
export * from './header/mod.tsx'
export * from './icon/mod.tsx'
export * from './icon_button/mod.tsx'
export * from './image/mod.tsx'
export * from './label/mod.tsx'
export * from './modal/mod.tsx'
export * from './nested_flow/mod.tsx'
export * from './padding/mod.tsx'
export * from './preview_box/mod.tsx'
export * from './radio_input/mod.tsx'
export * from './scrollable_box/mod.tsx'
export * from './sidebar_layout/mod.tsx'
export * from './skeleton/mod.tsx'
export * from './table/mod.tsx'
export * from './text_input/mod.tsx'
export * from './update_boundary/mod.tsx'
