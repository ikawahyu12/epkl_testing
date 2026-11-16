import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.llm.keyword.LlmKeywords as LLM
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser(null)

WebUI.navigateToUrl('http://localhost/epkl_website/e-pkl/admin-web/public/?page=manajemen-admin')

WebUI.setText(findTestObject('Manajemen_User/Page_Login Admin  E-PKL/input_Masukkan username atau email'), 'Afredo')

WebUI.setEncryptedText(findTestObject('Manajemen_User/Page_Login Admin  E-PKL/input_Masukkan password'), 'e49H4Kf1KPNwQBO6NHxxDA==')

WebUI.click(findTestObject('Manajemen_User/Page_Login Admin  E-PKL/i_far fa-eye'))

WebUI.click(findTestObject('Manajemen_User/Page_Login Admin  E-PKL/button_Masuk'))

WebUI.click(findTestObject('Manajemen_User/Page_Dashboard Monitoring PKL  E-PKL/span_Manajemen User'))

WebUI.mouseOver(findTestObject('Manajemen_User/Page_Dashboard Monitoring PKL  E-PKL/li_Manajemen User'))

WebUI.click(findTestObject('Manajemen_User/Page_Dashboard Monitoring PKL  E-PKL/a_Manajemen Admin'))

WebUI.click(findTestObject('Manajemen_User/Page_Manajemen Admin  E-PKL/a_Edit'))

WebUI.setText(findTestObject('Manajemen_User/Page_Edit Admin  E-PKL/input_Masukkan nama lengkap admin'), 'Valentino Delmade Edit')

WebUI.click(findTestObject('Manajemen_User/Page_Edit Admin  E-PKL/button_Perbarui Admin'))

WebUI.delay(2)
WebUI.verifyTextPresent('Valentino Delmade Edit', false)
WebUI.closeBrowser()
