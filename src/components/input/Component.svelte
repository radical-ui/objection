<script>
	import { createEventDispatcher, onMount, tick, afterUpdate } from 'svelte';
	import { MDCTextField } from '@material/textfield/index';
	import MDCStyles from '../Style.svelte';

	export let value = null;
	export let placeholder = null;
	export let helper = null;
	export let type = 'text';
	export let rows = 1;
	export let compress = true;
	export let color = 'var(--inputs)';
	export let background = 'var(--inputs-background)';
	export let backgroundHover = 'var(--inputs-background-hover)';
	export let backgroundFocus = 'var(--inputs-background-focus)';
	export let placeholderColor = 'var(--inputs-placeholder)';
	export let outlineColor = 'var(--inputs-outline)';
	export let outlineHover = 'var(--inputs-outline-hover)';
	export let outlined = false;
	export let resize = false;
	export let allowEnter = false;
	export let isFocused = false;
	export let maxChars = null;
	export let error = null;
	export let disabled = false;
	export let block = false;
	export let showErrorIcon = true;
	export let trim = false;
	export let leadingIcon = null;
	export let trailingIcon = null;
	export let leadingIconColor = 'var(--inputs-placeholder)';
	export let trailingIconColor = 'var(--inputs-placeholder)';
	export let leadingIconIsButton = false;
	export let trailingIconIsButton = false;
	export let leadingIconHovering = false;
	export let trailingIconHovering = false;

	let textField;
	let startUpEl;
	let inputEl;
	let hovering = false;
	let errors = 'var(--all-errors)';
	let randomId = 'input-component-' + Math.random() * Math.random();
	let showTextarea = rows > 1 || rows == null;
	let remSize = parseInt(getComputedStyle(document.documentElement).fontSize);
	let rowSize = remSize * 1.75;
	let padding = rows == 1 ? 30 : 16;
	let maxSize = rowSize * rows + padding;
	let floatAboveStart = false;
	let outlinedElement;
	let labelOutlined;

	const dispatch = createEventDispatcher();

	onMount(async () => {
		if (maxChars != null) inputEl.setAttribute('maxlength', maxChars);
		textField = new MDCTextField(startUpEl);
		if (isFocused) textField.focus();

		resizeTextarea(inputEl);
		if (value != null && value != undefined && value != '') {
			inputEl.value = value;
			floatAboveStart = true;

			if (outlined) {
				await tick();
				outlinedElement.style.width = addPx(
					labelOutlined.offsetWidth * 0.75 + 8
				);
			}
		}
	});

	afterUpdate(() => {
		if (textField) {
			if (isFocused) textField.focus();
			else inputEl.blur();
		}
	});

	function resizeTextarea(el) {
		if (compress) {
			if (rows != null) {
				if (el.scrollHeight > maxSize) return;
			}

			if (el.value == '') el.style.height = addPx(rowSize + padding);
			else {
				el.style.height = 'auto';
				el.style.height = addPx(el.scrollHeight);
			}
		} else {
			el.style.height = addPx(maxSize);
		}
	}

	function addPx(val) {
		return `${val}px`;
	}

	// Events
	function handleInput(e) {
		value = event.target.value;
		resizeTextarea(e.target);
		dispatch('valuechanged', e.target.value);
	}
	function handleKeyup(e) {
		// void
	}
	async function handleKeydown(e) {
		dispatch('prevaluechanged', value);
		dispatch('keypress', e);
		if (!allowEnter && e.key == 'Enter' && !e.shiftKey) {
			e.preventDefault();
			await tick();
			dispatch('done', value);
			isFocused = false;
		}
	}
	function handleFocus(e) {
		isFocused = true;
		dispatch('focus');
	}
	function handleBlur(e) {
		isFocused = false;
		dispatch('blur');
	}
	function handleMouseover(e) {
		hovering = true;
	}
	function handleMouseleave(e) {
		hovering = false;
	}
</script>

<style>
	.mdc-custom-inline-block {
		display: inline-block;
		width: 300px;
	}
	.mdc-custom-container {
		width: 100%;
	}
	.mdc-custom-error {
		font-family: Roboto, sans-serif;
		-moz-osx-font-smoothing: grayscale;
		-webkit-font-smoothing: antialiased;
		font-size: 0.75rem;
		line-height: 1.25rem;
		font-weight: 400;
		letter-spacing: 0.0333333333em;
		text-decoration: inherit;
		text-transform: inherit;
		display: block;
		margin-top: 0;
		line-height: normal;
		margin: 0;
		color: var(--all-errors);
	}
	.mdc-custom-error::before {
		display: inline-block;
		width: 0;
		height: 16px;
		content: '';
		vertical-align: 0;
	}
	.mdc-custom-no-resize {
		resize: none;
	}
	.mdc-custom-no-scroll {
		overflow: hidden;
	}
</style>

<MDCStyles />

<div class="over" class:mdc-custom-inline-block={!block}>
	<div
		class="mdc-text-field mdc-custom-container"
		bind:this={startUpEl}
		class:mdc-text-field--textarea={showTextarea}
		class:mdc-text-field--no-label={!placeholder}
		class:mdc-text-field--disabled={disabled}
		class:mdc-text-field--with-leading-icon={leadingIcon}
		class:mdc-text-field--with-trailing-icon={(error && showErrorIcon) || trailingIcon}
		class:mdc-text-field--outlined={outlined}
		class:input-disabled={disabled}
		class:input-focused={isFocused}
		class:input-error={error}
		class:input-else={!error && !isFocused && !disabled}
		style="background-color: {outlined ? 'unset' : isFocused ? backgroundFocus : hovering ? backgroundHover : background};"
		on:mouseover={handleMouseover}
		on:mouseleave={handleMouseleave}>

		{#if leadingIcon}
			<i
				class="material-icons mdc-text-field__icon leading-icon"
				tabindex="0"
				on:click={(e) => dispatch('leadingiconclick', e)}
				on:mouseover={(e) => {
					leadingIconHovering = true;
					dispatch('leadingiconhover', e);
				}}
				on:mouseout={(e) => (leadingIconHovering = false)}
				style="{!leadingIconIsButton ? 'z-index: -1;' : ''}color: {leadingIconColor}">
				{leadingIcon}
			</i>
		{/if}

		{#if showTextarea}
			<textarea
				bind:this={inputEl}
				class="mdc-text-field__input"
				class:mdc-custom-no-resize={!resize}
				class:mdc-custom-no-scroll={rows == null}
				style="caret-color: {error ? errors : color};"
				id={randomId}
				rows={rows == null || compress ? 1 : rows}
				{disabled}
				on:input={handleInput}
				on:keyup={handleKeyup}
				on:keydown={handleKeydown}
				on:focus={handleFocus}
				on:blur={handleBlur} />
		{:else}
			<input
				{type}
				bind:this={inputEl}
				class="mdc-text-field__input"
				style="caret-color: {error ? errors : color}"
				{disabled}
				id={randomId}
				on:input={handleInput}
				on:keyup={handleKeyup}
				on:keydown={handleKeydown}
				on:focus={handleFocus}
				on:blur={handleBlur} />
		{/if}

		{#if placeholder != null && !outlined}
			<label
				class="mdc-floating-label"
				class:mdc-floating-label--float-above={floatAboveStart}
				class:mdc-floating-label__float-above={floatAboveStart}
				for={randomId}
				style="color: {error ? errors : isFocused ? color : placeholderColor}">
				{placeholder}
			</label>
		{/if}

		{#if error && showErrorIcon}
			<i
				class="material-icons mdc-text-field__icon error-icon"
				tabindex="0"
				style="color: {errors}">
				error
			</i>
		{:else if trailingIcon}
			<i
				class="material-icons mdc-text-field__icon trailing-icon"
				tabindex="0"
				on:click={(e) => dispatch('trailingiconclick', e)}
				on:mouseover={(e) => {
					trailingIconHovering = true;
					dispatch('trailingiconhover', e);
				}}
				on:mouseout={(e) => (trailingIconHovering = false)}
				style="{!trailingIconIsButton ? 'z-index: -1;' : ''}color: {trailingIconColor}">
				{trailingIcon}
			</i>
		{/if}

		{#if outlined}
			<div
				class="mdc-notched-outline"
				class:mdc-notched-outline--notched={floatAboveStart}>
				<div
					class="mdc-notched-outline__leading"
					style="border-color: {isFocused ? color : hovering ? outlineHover : outlineColor}" />
				{#if placeholder != null}
					<div
						class="mdc-notched-outline__notch"
						style="border-color: {isFocused ? color : hovering ? outlineHover : outlineColor}"
						bind:this={outlinedElement}>
						<label
							for={randomId}
							bind:this={labelOutlined}
							class="mdc-floating-label"
							class:mdc-floating-label--float-above={floatAboveStart}
							class:mdc-floating-label__float-above={floatAboveStart}
							style="color: {isFocused ? color : placeholderColor}">
							{placeholder}
						</label>
					</div>
				{/if}
				<div
					class="mdc-notched-outline__trailing"
					style="border-color: {isFocused ? color : hovering ? outlineHover : outlineColor}" />
			</div>
		{:else}
			<div
				class="mdc-line-ripple"
				style="background-color: {error ? errors : color}" />
		{/if}
	</div>

	{#if !trim}
		<div class="mdc-text-field-helper-line">
			{#if error == null}
				<div class="mdc-text-field-helper-text" aria-hidden={false}>
					{#if helper}{helper}{/if}
				</div>
			{/if}
			<div class="mdc-custom-error">
				{#if error}{error}{/if}
			</div>

			{#if maxChars != null}
				<div class="mdc-text-field-character-counter" />
			{/if}
		</div>
	{/if}
</div>
